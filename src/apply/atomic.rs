use std::path::Path;
use crate::Error;

/// Write `content` to `path` atomically.
///
/// Writes to a temporary file in the same directory, then renames.
/// Rename is atomic on POSIX filesystems — if the process is killed
/// between the write and rename, the original file is untouched.
pub fn write(path: &Path, content: &str) -> Result<(), Error> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let tmp = dir.join(format!(
        ".llm-apply-{}.tmp",
        path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("file")
    ));

    std::fs::write(&tmp, content)?;
    std::fs::rename(&tmp, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn writes_content_correctly() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.rs");
        std::fs::write(&path, "original").unwrap();

        write(&path, "updated").unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "updated");
    }

    #[test]
    fn no_tmp_file_left_on_success() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.rs");
        std::fs::write(&path, "original").unwrap();

        write(&path, "updated").unwrap();

        let tmp = dir.path().join(".llm-apply-test.rs.tmp");
        assert!(!tmp.exists(), "tmp file should not exist after successful write");
    }
}
