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

/// Read the exact bytes a decoder should consume, verifying their saved digest.
/// The explicit byte budget prevents unbounded buffering of an original. Do not
/// reopen the path after this check; pass these bytes directly to the decoder.
pub fn read_verified(root: &Path, relative: &str, expected: &str, max_bytes: u64)
    -> Result<Vec<u8>, String> {
    validate_digest(expected)?;
    if max_bytes == 0 || max_bytes > usize::MAX as u64 - 1 {
        return Err("Invalid original byte budget".into());
    }
    let path = resolve_original(root, relative)?;
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let metadata = file.metadata().map_err(|e| e.to_string())?;
    if !metadata.is_file() || metadata.len() > max_bytes {
        return Err("Original is not a regular file within byte budget".into());
    }
    let mut bytes = Vec::new();
    let mut reader = file.take(max_bytes + 1);
    let mut buffer = [0u8; 65536];
    loop {
        let count = reader.read(&mut buffer).map_err(|e| e.to_string())?;
        if count == 0 { break; }
        if bytes.len() as u64 + count as u64 > max_bytes {
            return Err("Original exceeds byte budget".into());
        }
        bytes.try_reserve(count).map_err(|_| "Cannot allocate original buffer".to_string())?;
        bytes.extend_from_slice(&buffer[..count]);
    }
    if format!("{:x}", Sha256::digest(&bytes)) != expected {
        return Err("Original differs from saved asset identity; re-import before export".into());
    }
    Ok(bytes)
}

fn validate_digest(expected: &str) -> Result<(), String> {
    if expected.len() != 64 || !expected.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) {
        return Err("Expected SHA256 must be 64 lowercase hexadecimal characters".into());
    }
    Ok(())
}

/// Refuse stale originals before export instead of silently rendering changed
/// bytes under a saved asset identity. A later decoder must still verify its
/// own opened bytes; this check alone cannot close a subsequent open/read race.
pub fn verify_identity(root: &Path, relative: &str, expected: &str) -> Result<Fingerprint, String> {
    validate_digest(expected)?;
    let actual = fingerprint(root, relative)?;
    if actual.sha256 != expected {
        return Err("Original differs from saved asset identity; re-import before export".into());
    }
    Ok(actual)
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
        assert!(verify_identity(&root, "nested/旅.jpg", &identity.sha256).is_ok());
        let verified = read_verified(&root, "nested/旅.jpg", &identity.sha256, 3).unwrap();
        assert_eq!(verified, b"abc");
        assert!(read_verified(&root, "nested/旅.jpg", &identity.sha256, 2).is_err());
        assert!(verify_identity(&root, "nested/旅.jpg", "bad-hash").is_err());
        // Same length is insufficient: saved identity must detect changed bytes.
        std::fs::write(&file, b"abd").unwrap();
        assert_eq!(verified, b"abc"); // Decoder input remains the verified snapshot.
        assert!(read_verified(&root, "nested/旅.jpg", &identity.sha256, 3).is_err());
        assert!(verify_identity(&root, "nested/旅.jpg", &identity.sha256)
            .unwrap_err().contains("differs"));
        // Only remove the known fixture files/directories we created.
        std::fs::remove_file(file).unwrap();
        std::fs::remove_dir(root.join("nested")).unwrap();
        std::fs::remove_dir(root).unwrap();
    }
}
