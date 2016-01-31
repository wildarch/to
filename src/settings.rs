extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use std::convert::{From, Into};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub directories: Vec<String>
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            directories: Vec::new()
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let file = File::create(path);
        match file {
            Ok(mut file) => {
                let buffer: String = self.into();
                file.write_all(buffer.as_bytes()).unwrap();
                Ok(())
            },
            Err(e) => Err(e)
        }
    }
}

impl From<File> for Settings {
    fn from(mut file: File) -> Settings {
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        serde_json::from_str(&buffer).unwrap()
    }
}

impl<'a> Into<String> for &'a Settings {
    fn into(self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
