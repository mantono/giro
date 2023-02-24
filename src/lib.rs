use std::{
    fs::{DirEntry, ReadDir},
    path::PathBuf,
};

pub fn git_root<P: AsRef<std::path::Path>>(path: P) -> Result<Option<PathBuf>, std::io::Error> {
    let path: PathBuf = path.as_ref().canonicalize()?;
    let handle: ReadDir = std::fs::read_dir(&path)?;

    for entry in handle.into_iter() {
        let entry: DirEntry = match entry {
            Ok(entry) => entry,
            Err(e) => return Err(e),
        };

        if !is_dir(&entry) {
            continue;
        }

        if !is_git_dir(&entry)? {
            continue;
        }

        let has_config: bool = has_git_config(&entry)?;

        if has_config {
            return Ok(Some(path));
        }
    }

    match path.parent() {
        Some(parent) => git_root(parent.to_str().unwrap()),
        None => Ok(None),
    }
}

fn is_dir(entry: &DirEntry) -> bool {
    match entry.metadata() {
        Ok(md) => md.is_dir(),
        Err(_) => false,
    }
}

fn is_git_dir(entry: &DirEntry) -> Result<bool, std::io::Error> {
    match entry.file_name().to_str() {
        Some(".git") => has_git_config(entry),
        _ => Ok(false),
    }
}

fn has_git_config(entry: &DirEntry) -> Result<bool, std::io::Error> {
    let handle: ReadDir = std::fs::read_dir(entry.path())?;
    let found: bool =
        handle
            .into_iter()
            .filter_map(|e| e.ok())
            .any(|e| match e.file_name().to_str() {
                Some("config") => true,
                _ => false,
            });

    Ok(found)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dev_is_not_git() {
        let git_root: Option<PathBuf> = git_root("/dev").unwrap();
        assert_eq!(None, git_root);
    }

    #[test]
    fn test_project_has_git_root() {
        let git_root: Option<PathBuf> = git_root(".").unwrap();
        assert!(git_root.is_some());
    }
}
