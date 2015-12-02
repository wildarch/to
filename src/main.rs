use std::env;
use std::fs;
use std::path::PathBuf;
use std::vec;

const WORKSPACE_PATH: &'static str = "/home/daan/workspace";

fn main() {
    let mut args = env::args();
    args.next();    //Skip first argument
    let mode = args.next();
    match mode {
        Some(mode) => {
            let query = args.next();
            match query {
                Some(query) => {
                    let query_path = PathBuf::from(&query);
                    
                    if query_path.exists() {
                        println!("{}", query);
                        if mode == "go" { return; }    
                    }
                    let mut query_root = query_path.clone();
                    let query_name = match query_path.file_name() {
                        Some(os_string) => { 
                            query_root.pop();
                            os_string.to_str().unwrap() 
                        },
                        None => ""    
                    }; 
                    let workspace_iter = match query_root.has_root() {
                        false => search_dir(query_name, PathBuf::from(WORKSPACE_PATH), mode == "go"),
                        true => Vec::new().into_iter()   
                    };
                    let base_iter = search_dir(query_name, query_root, true);
                    let mut total_iter = workspace_iter.chain(base_iter);
                    if mode == "go" {
                        match total_iter.next() {
                            Some(string) => return println!("{}", string),
                            None => return    
                        }   
                    }
                    for string in total_iter {
                        println!("{}", string);    
                    }
                },
                None => return    
            }
        },
        None => return    
    }
    
}

fn search_dir(query: &str, dir: PathBuf, full_path: bool) -> vec::IntoIter<String> {
    let iter = match fs::read_dir(dir.as_path()) {
        Ok(it) => it,
        Err(_) => return Vec::new().into_iter()
    };
    let mut dirs = Vec::new();
    for item in iter {
        let name = String::from(item.unwrap().file_name().to_str().unwrap());
        if name.starts_with(query){ 
            if full_path {
                let mut path = PathBuf::new();
                path.push(dir.as_path());
                path.push(name);
                dirs.push(String::from(path.to_str().unwrap()))   
            }
            else {
                dirs.push(name)    
            }
        }
    }
    return dirs.into_iter()
}