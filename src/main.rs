use std::env;
use std::fs;
use std::path::{PathBuf};
use std::io::Error;

const WORKSPACE_PATH: &'static str = "/home/daan/workspace/";

fn main() {
    let mut args = env::args();
    args.next();    //Skip first argument
    let mode = args.next();
    match mode {
        Some(mode) => {
            match args.next() {
                Some(query) => {
                    let query_path = PathBuf::from(&query);
                    let base_iter = path_match(&query, true).unwrap_or(Box::new(Vec::new().into_iter()));
                    
                    let mut total_iter: Box<Iterator<Item=String>> = match query_path.has_root() {
                        true => Box::new(base_iter),
                        false => {
                            let mut workspace_path = PathBuf::from(WORKSPACE_PATH);
                            workspace_path.push(&query_path);
                            let workspace_iter = path_match(workspace_path.to_str().unwrap(), mode == "go").unwrap_or(
                                    Box::new(Vec::new().into_iter())
                                ).chain(base_iter);
                            Box::new(workspace_iter)
                        },
                        
                    };
                    
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
                None => {
                    if mode != "list" { return }
                    for string in path_match(WORKSPACE_PATH, false).unwrap() {
                        println!("{}", string);    
                    }
                }   
            }
        },
        None => return println!("No mode")    
    }
    
}

fn path_match(query_path: &str, full_path: bool) -> Result<Box<Iterator<Item=String>>, Error> {
    let mut query_buf = PathBuf::from(query_path);
    let clone = query_buf.clone();
    
    let query = match clone.file_name() {
          Some(file_name) => {
              //Handle queries that are relative but are not prefixed with ./
              if query_buf.components().count() == 1 {
                      query_buf = PathBuf::from("./");
                      String::from(query_path)
              }
              //Handle queries that end in /
              else if query_path.ends_with("/") { 
                  String::from("")
              }
              else {
                  query_buf.pop(); //Pop filename
                  String::from(file_name.to_str().unwrap_or(""))
              }
          },
          None => {
              String::from("")    
          }
    };
    
    let mut base = Vec::new();
    if query_buf.exists() && query.is_empty() {
        base.push(String::from(query_buf.to_str().unwrap()));
    }
    
    Ok(
        Box::new(
            base.into_iter().chain(
                try!(fs::read_dir(query_buf.as_path())).filter_map(move |dir|{
            
                    let dir = dir.unwrap();
                    let mut dir_name = String::from(dir.file_name().to_str().unwrap());
                    if dir_name.starts_with(&query) && dir.file_type().unwrap().is_dir() {
                        match full_path {
                            true => {
                                let mut full_path = String::from(dir.path().to_str().unwrap());
                                full_path.push('/');
                                Some(full_path)
                            },
                            false => {
                                dir_name.push('/');
                                Some(dir_name)
                            }
                        }    
                    }
                    else { 
                        None   
                    }
                })
            )
        )
    )
}