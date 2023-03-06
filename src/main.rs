use giro::git_root;

fn main() {
    let mut args = std::env::args();
    let path: String = args.nth(1).unwrap_or_else(|| String::from("."));
    match git_root(path) {
        Ok(dir) => match dir {
            Some(root) => println!("{}", root.to_str().unwrap()),
            None => std::process::exit(1),
        },
        Err(e) => {
            eprintln!("error: {}", e.kind());
            std::process::exit(2);
        }
    }
}
