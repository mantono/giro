use std::{
    io::Error,
    path::{Path, PathBuf},
};

/// Check if a path resides within a git repository, and if so, return the
/// path to the root of the git repository. If the path does not reside insdie
/// a git repository, `None` will be returned.
///
/// The ".git" directory inside a git repository is also considered to be
/// inside the git repository.
pub fn git_root<P: AsRef<Path>>(path: P) -> Result<Option<PathBuf>, Error> {
    let path: PathBuf = normalize(path)?;
    Ok(traverse(&path))
}

fn normalize<P: AsRef<Path>>(path: P) -> Result<PathBuf, Error> {
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

fn traverse(path: &Path) -> Option<PathBuf> {
    let git_config: PathBuf = path.join(".git").join("config");
    if git_config.exists() {
        Some(path.to_path_buf())
    } else {
        match path.parent() {
            Some(parent) => traverse(parent),
            None => None,
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

    #[test]
    fn test_project_git_config_dir_is_inside_git_repo() {
        let git_root: Option<PathBuf> = git_root(".git/").unwrap();
        assert!(git_root.is_some());
    }
}
