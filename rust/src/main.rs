extern crate serde;
extern crate serde_json;

use std::env;
use std::path::{Path, PathBuf};
use std::fs::{File, DirEntry};
use std::process::exit;

mod settings;
use settings::Settings;

const CONF_PATH: &'static str = ".config/to"; //Relative from the user's home directory

fn get_conf_path() -> PathBuf {
    let mut conf_path = env::home_dir().unwrap();
    conf_path.push(CONF_PATH);
    return conf_path;
}

fn get_settings() -> Option<Settings> {
    let conf_path = get_conf_path();
    let file = File::open(&conf_path);
    match file {
        Ok(file) => match Settings::from_file(file) {
            Ok(settings) => Some(settings),
            Err(_) => {
                println!("Error: {} is not valid", conf_path.to_str().unwrap());
                None
            }
        },
        Err(_) => {
            let s = Settings::new();
            save_settings(&s);
            return Some(s);
        },
    }
}



fn main() {
    let mut args = env::args();
    args.next();    // Skip the first argument (the command path)

    let mode = match args.next() {
        Some(mode) => mode,
        None => return println!("Usage: to go|list|add|remove|dirs|version [query|dir]")
    };

    let settings = match get_settings() {
        Some(settings) => settings,
        None => {
            exit(1);
        }
    };

    let query = args.next().unwrap_or_default();

    match mode.as_ref() {
        "add" => {
            if query.is_empty() {
                let cwd = env::current_dir()
                    .unwrap().to_str()
                    .unwrap().to_owned();
                add(cwd, settings);
            } else {
                add(query, settings);
            }
            return;
        }
        "remove" => {
            if !query.is_empty() {
                remove(&query, settings);
            }
            return;
        }
        "dirs" => {
            list_dirs(settings);
            return;
        }
        "version" => {
            println!("To: Rust version");
            return;
        }
        // list, go or an invalid mode
        _ => {}
    }

    let mut results = settings.directories.into_iter()
        .filter_map(|dir| Path::new(&dir).read_dir().ok())
        .flat_map(|entries| {
            entries.filter_map(|entry| {
                if let Ok(entry) = entry {
                    if entry.path().is_dir() && entry.file_name().to_str().unwrap().starts_with(&query) {
                        Some(entry)
                    }
                    else {
                        None
                    }
                }
                else {
                    None
                }
            })
        });

    match mode.as_ref() {
        "go" => go(&mut results),
        "list" => list(&mut results),
        _ => println!("Invalid mode specified!"),
    }
}

fn go<I: Iterator<Item=DirEntry>>(results: &mut I) {
    match results.min_by_key(|e| Some(e.file_name().len())) {
        Some(dir) => println!("{}", dir.path().to_str().unwrap()),
        None => println!("Error: no results found"),
    }
}

fn list<I: Iterator<Item=DirEntry>>(results: &mut I) {
    let mut sorted: Vec<DirEntry> = results.collect();
    sorted.sort_by_key(|x| x.file_name().len());
    for dir in sorted {
        println!("{}", dir.file_name().to_str().unwrap());
    }
}

fn add(path: String, mut settings: Settings) {
    let p = Path::new(&path);
    if p.is_dir() && p.exists() {
        settings.directories.push(path.clone());
        save_settings(&settings);
    }
    else {
        println!("Error: not a valid directory");
    }
}

fn remove(index: &str, mut settings: Settings) {
    let index: usize = match index.parse() {
        Ok(index) => index,
        Err(_) => {
            println!("Error: {} is not a valid index", index);
            return list_dirs(settings);
        }
    };
    if index >= settings.directories.len() {
        println!("Error: {} is not a valid index", index);
        return list_dirs(settings);
    }
    settings.directories.remove(index);
    save_settings(&settings);
}

fn list_dirs(settings: Settings) {
    if settings.directories.len() == 0 {
        println!("Error: No directories in settings file");
        return;
    }
    for (index, dir) in settings.directories.iter().enumerate() {
        println!("[{}] {}", index, dir);
    }
}


fn save_settings(settings: &Settings) {
    let conf_path = get_conf_path();
    match settings.save(&conf_path) {
        Ok(_) => println!("Settings saved"),
        Err(e) => println!("Error saving settings: {}", e),
    }
}
