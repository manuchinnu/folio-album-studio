//! Publish complete new files without overwriting existing user work.
//! A hard link makes the destination visible only after the staged bytes sync.
//! Filesystems without hard-link support fail rather than copying partially.
use std::{fs::{self, OpenOptions}, io::{self, Write}, path::Path,
    sync::atomic::{AtomicU64, Ordering}};

static NEXT: AtomicU64 = AtomicU64::new(0);

/// Parent directory must already exist. Does not follow an existing destination
/// symlink or replace any existing file. Not a hostile-directory security API.
/// File bytes are synced; directory-entry durability across power loss is not
/// guaranteed. A crash may leave a harmless staging file for later recovery.
pub fn write_new(destination: &Path, bytes: &[u8]) -> io::Result<()> {
    let parent = destination.parent().filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut staged = None;
    for _ in 0..128 {
        let path = parent.join(format!(".folio-stage-{}-{}", std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => { staged = Some((path, file)); break; }
            Err(e) if e.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(e) => return Err(e),
        }
    }
    let (path, mut file) = staged.ok_or_else(|| io::Error::new(
        io::ErrorKind::AlreadyExists, "Cannot reserve staging file"))?;
    let result = file.write_all(bytes).and_then(|_| file.sync_all());
    drop(file);
    let result = result.and_then(|_| fs::hard_link(&path, destination));
    // Publication success must not become a retryable failure just because
    // cleanup failed: destination already contains the complete bytes.
    let _ = fs::remove_file(&path);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn concurrent_publishers_never_replace_or_mix_contents() {
        let root = std::env::temp_dir().join(format!("folio-publish-{}-{}",
            std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir(&root).unwrap();
        let destination = root.join("project.json");
        let results = std::thread::scope(|scope| {
            let handles: Vec<_> = (0..8u8).map(|value| {
                let destination = &destination;
                scope.spawn(move || (value, write_new(destination, &vec![value; 8192])))
            }).collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>()
        });
        let winners: Vec<_> = results.iter().filter(|(_,r)| r.is_ok()).collect();
        assert_eq!(winners.len(), 1);
        assert_eq!(fs::read(&destination).unwrap(), vec![winners[0].0; 8192]);
        assert!(write_new(&destination, b"replacement").is_err());
        assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
        fs::remove_file(destination).unwrap();
        fs::remove_dir(root).unwrap();
    }
}
