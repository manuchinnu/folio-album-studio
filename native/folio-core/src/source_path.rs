//! Resolve project-relative originals without allowing a project to escape its
//! selected source root. No file writes and no network access.
use std::path::{Path, PathBuf};
use std::io::Read;
use sha2::{Digest, Sha256};

#[derive(Debug, serde::Serialize)]
pub struct Fingerprint {
    pub sha256: String,
    pub bytes: u64,
}

/// Streaming identity check for local originals. Metadata checks catch ordinary
/// edits during hashing; this does not claim protection against hostile races.
pub fn fingerprint(root: &Path, relative: &str) -> Result<Fingerprint, String> {
    let path = resolve_original(root, relative)?;
    let mut file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let before = file.metadata().map_err(|e| e.to_string())?;
    if !before.is_file() { return Err("Original is not a regular file".into()); }
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 65536];
    let mut bytes = 0u64;
    loop {
        let count = file.read(&mut buffer).map_err(|e| e.to_string())?;
        if count == 0 { break; }
        hasher.update(&buffer[..count]);
        bytes += count as u64;
    }
    let after = file.metadata().map_err(|e| e.to_string())?;
    if bytes != before.len() || after.len() != before.len()
        || before.modified().map_err(|e| e.to_string())? != after.modified().map_err(|e| e.to_string())? {
        return Err("Original changed while hashing; retry import".into());
    }
    Ok(Fingerprint { sha256: format!("{:x}", hasher.finalize()), bytes })
}

pub fn resolve_original(root: &Path, relative: &str) -> Result<PathBuf, String> {
    // Apply the same portable grammar on Windows and Unix. Reject Windows ADS,
    // drive/UNC paths and backslash traversal even when running on Unix.
    if relative.is_empty() || relative.len() > 500 || relative.contains(['\\', ':', '\0'])
        || relative.split('/').any(|part| part.is_empty() || part == "." || part == ".."
            || part.ends_with(' ') || part.ends_with('.')) {
        return Err("Original must be a portable relative file path".into());
    }
    let root = root.canonicalize().map_err(|e| format!("Cannot resolve source root: {e}"))?;
    if !root.is_dir() { return Err("Source root is not a directory".into()); }
    let candidate = root.join(relative).canonicalize().map_err(|e| format!("Cannot resolve original: {e}"))?;
    if !candidate.starts_with(&root) { return Err("Original resolves outside selected source root".into()); }
    if !candidate.is_file() { return Err("Original is not a regular file".into()); }
    Ok(candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_nonportable_and_traversal_paths_before_access() {
        for path in ["", "../a.jpg", "a/../b.jpg", "/a.jpg", "C:/a.jpg", "a\\b.jpg", "a.jpg:stream", "a//b.jpg", "./a.jpg", "a./b.jpg", "a /b.jpg"] {
            assert!(resolve_original(Path::new("does-not-exist"), path).unwrap_err().contains("portable"), "{path}");
        }
    }

    #[test]
    fn resolves_nested_unicode_and_rejects_missing_or_directory() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts")
            .join(format!("path-test-{}", std::process::id()));
        std::fs::create_dir_all(root.join("nested")).unwrap();
        let file = root.join("nested/旅.jpg");
        std::fs::write(&file, b"synthetic path fixture").unwrap();
        assert_eq!(resolve_original(&root, "nested/旅.jpg").unwrap(), file.canonicalize().unwrap());
        assert!(resolve_original(&root, "missing.jpg").is_err());
        assert!(resolve_original(&root, "nested").is_err());
        std::fs::write(&file, b"abc").unwrap();
        let identity = fingerprint(&root, "nested/旅.jpg").unwrap();
        assert_eq!(identity.bytes, 3);
        assert_eq!(identity.sha256, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        // Only remove the known fixture files/directories we created.
        std::fs::remove_file(file).unwrap();
        std::fs::remove_dir(root.join("nested")).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
}
