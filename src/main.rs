use is_git::git_root;

fn main() {
    let mut args = std::env::args();
    let path = args.nth(1).unwrap_or(String::from("."));
    match git_root(path) {
        Ok(dir) => match dir {
            Some(root) => println!("{:?}", root),
            None => print!("Not a git dir"),
        },
        Err(e) => eprintln!("Unable to check path: {}", e),
    }
}
