# git-root
This is a crate to for resolving the Git repository root directory, if any, of a path. So essentially,
it enables you to find out two things:
- Does the path X belong to a Git repository?
- If so, what is the root directory of this Git repository?

This crate only exposes one public function: 'git_root'

```rust
pub fn git_root<P: AsRef<std::path::Path>>(path: P) -> Result<Option<PathBuf>, std::io::Error>
```

Having this isolated functionality in a create means that other applications or crates may be able
to use this crate over for example git2 or similar crates when they are only used for this
functionality.