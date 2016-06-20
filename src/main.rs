#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;

mod directory;
use directory::{find_in, Directory};

mod settings;
use settings::Settings;

const CONF_PATH: &'static str = ".config/to"; //Relative from the user's home directory


fn main() {
    let mut args = env::args();
    args.next();    // Skip the first argument (the command path)

    let mode = match args.next() {
        Some(mode) => mode,
        None => return println!("Mode not specified!"),
    };

    let settings = get_settings();

    let query_string = args.next().unwrap_or_default();
    let query = PathBuf::from(&query_string);

    match mode.as_ref() {
        "add" => {
            if query_string.is_empty() {
                let cwd = env::current_dir().unwrap();
                add(&cwd, settings);
            } else {
                add(&query, settings);
            }
            return;
        }
        "remove" => {
            remove(&query_string, settings);
            return;
        }
        "dirs" => {
            list_dirs(settings);
            return;
        }
        // list, go or an invalid mode
        _ => {}
    }

    let mut results = settings.directories.into_iter().flat_map(move |dir| {
        let path = Path::new(&dir);
        find_in(&query_string, path).unwrap()
    });

    match mode.as_ref() {
        "go" => go(&mut results),
        "list" => list(&mut results),
        _ => println!("Invalid mode specified!"),
    }
}

type ResultIter = Iterator<Item = Directory>;

fn go(results: &mut ResultIter) {
    match results.next() {
        Some(dir) => println!("{}", dir.path_str()),
        None => println!("No results found!"),
    }
}

fn list(results: &mut ResultIter) {
    for item in results {
        println!("{}", item.name);
    }
}

fn add(path: &Path, mut settings: Settings) {
    if path.is_dir() {
        let string = if path.is_absolute() {
            String::from(path.to_str().unwrap())
        } else {
            let mut cwd = env::current_dir().unwrap();
            cwd.push(path);
            String::from(cwd.as_path().to_str().unwrap())
        };
        if settings.directories.contains(&string) {
            println!("Directory {} already in list:", &string);
            return list_dirs(settings);
        }
        println!("Directory '{}' added", &string);
        settings.directories.push(string);
        save_settings(settings);
    } else {
        println!("Invalid path specified!");
    }
}

fn remove(index: &str, mut settings: Settings) {
    let index: usize = match index.parse() {
        Ok(index) => index,
        Err(_) => {
            println!("Please specify the index of the element to remove: ");
            return list_dirs(settings);
        }
    };
    if index >= settings.directories.len() {
        println!("There is no directory at index {}: ", index);
        return list_dirs(settings);
    }
    settings.directories.remove(index);
    save_settings(settings);
}

fn list_dirs(settings: Settings) {
    for (index, dir) in settings.directories.iter().enumerate() {
        println!("[{}] {}", index, dir);
    }
}

fn get_settings() -> Settings {
    let mut conf_path = env::home_dir().unwrap();
    conf_path.push(CONF_PATH);
    let file = File::open(&conf_path);
    match file {
        Ok(file) => Settings::from_file(file).unwrap_or(Settings::new()),
        Err(_) => Settings::new(),
    }
}

fn save_settings(settings: Settings) {
    let mut conf_path = env::home_dir().unwrap();
    conf_path.push(CONF_PATH);
    match settings.save(&conf_path) {
        Ok(_) => println!("Settings saved"),
        Err(e) => println!("Error saving settings: {}", e),
    }
}
