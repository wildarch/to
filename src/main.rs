use std::env;
use std::path::Path;

mod directory;
use directory::{find_in, DirIter};

fn main(){
    let mut args = env::args();
    args.next();    // Skip the first argument (the command path)

    let mode = match args.next() {
        Some(mode) => mode,
        None => return println!("Mode not specified!")
    };
    let query = args.next().unwrap_or_default();
    let query = Path::new(&query);

    let base_path = Path::new("/home/daan/workspace");
    let results = find_in(query, base_path).unwrap();

    match mode.as_ref() {
        "list" => list(results),
        "go" => go(results),
        _ => println!("Invalid mode!")
    }
}

fn list(iter: DirIter) {
    for item in iter {
        println!("{}", item.name);
    }
}

fn go(mut iter: DirIter) {
    match iter.next() {
        Some(dir) => println!("{}", dir.path_str()),
        None => println!("No results found!")
    }
}
