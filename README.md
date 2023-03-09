# giro
giro (from **gi**t-**ro**ot) is a [crate](https://crates.io/crates/giro) to for resolving the Git repository root directory, if any, of a path. So essentially,
it enables you to find out two things:
- Does a path belong to a Git repository?
- If so, what is the root directory of this Git repository?

This crate only exposes one public function: 'git_root'

```rust
pub fn git_root<P: AsRef<std::path::Path>>(path: P) -> Result<Option<PathBuf>, std::io::Error>
```

Having this isolated functionality in a create means that other applications or crates may be able
to use this crate over for example git2 or similar crates when they are only used for this
functionality.

## Usage
### Library (crate)
Run
```sh
cargo add giro
```

or add

```toml
giro = "0.1"
```

to your projects Cargo.toml.

Then call it with
```rust
let root: Option<std::path::PathBuf> = giro::git_root("some/path")?;
```

### Application (binary)
This will install giro as a git plugin. The binary will be installed as `git-root`.

```sh
cargo install giro
```

then run

```sh
git root
```

to check if the current directory is inside a git repository or not.

#### Man Pages
If you want man pages for the git-root binary, it can be added by copying the file `git-root.1.man`
to a directory in your [`$MANPATH`](https://www.howtoforge.com/linux-manpath-command).