use std::{
    fs::{DirEntry, ReadDir},
    path::PathBuf,
};

/// Check if a path resides within a git repository, and if so, return the
/// path to the root of the git repository. If the path does not reside insdie
/// a git repository, `None` will be returned.
///
/// The ".git" directory inside a git repository is also considered to be
/// inside the git repository.
pub fn git_root<P: AsRef<std::path::Path>>(path: P) -> Result<Option<PathBuf>, std::io::Error> {
    let path: PathBuf = normalize(path)?;
    let handle: ReadDir = std::fs::read_dir(&path)?;

    for entry in handle {
        let entry: DirEntry = entry?;
        if accept(entry)? {
            return Ok(Some(path));
        }
    }

    match path.parent() {
        Some(parent) => git_root(parent.to_str().unwrap()),
        None => Ok(None),
    }
}

fn accept(entry: DirEntry) -> Result<bool, std::io::Error> {
    if !is_dir(&entry) {
        Ok(false)
    } else if !is_git_dir(&entry)? {
        Ok(false)
    } else if !has_git_config(&entry)? {
        Ok(false)
    } else {
        Ok(true)
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
        handle.into_iter().filter_map(|e| e.ok()).any(|e| match e.file_name().to_str() {
            Some("config") => true,
            _ => false,
        });

    Ok(found)
}

fn normalize<P: AsRef<std::path::Path>>(path: P) -> Result<PathBuf, std::io::Error> {
    let path: PathBuf = path.as_ref().canonicalize()?;
    if path.is_dir() {
        Ok(path)
    } else {
        match path.parent() {
            Some(parent) => Ok(parent.to_path_buf()),
            None => Ok(path),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_directory_is_not_in_git_repo() {
        let git_root: Option<PathBuf> = git_root("/dev").unwrap();
        assert_eq!(None, git_root);
    }

    #[test]
    fn test_file_is_not_in_git_repo() {
        let git_root: Option<PathBuf> = git_root("/dev/null").unwrap();
        assert_eq!(None, git_root);
    }

    #[test]
    fn test_project_has_git_root() {
        let git_root: Option<PathBuf> = git_root(".").unwrap();
        assert!(git_root.is_some());
    }
}
