use git_root::git_root;

fn main() {
    let mut args = std::env::args();
    let path: String = args.nth(1).unwrap_or(String::from("."));
    match git_root(path) {
        Ok(dir) => match dir {
            Some(root) => println!("{}", root.to_str().unwrap().to_string()),
            None => std::process::exit(1),
        },
        Err(e) => {
            eprintln!("error: {}", e.kind());
            std::process::exit(2);
        }
    }
}
