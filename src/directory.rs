use std::path::{PathBuf, Path};
use std::io::Error;
use std::fs::read_dir;

#[derive(Debug)]
pub struct Directory {
    pub path: PathBuf,
    pub name: String
}

impl Directory {
    pub fn path_str(&self) -> &str{
        self.path.to_str().unwrap()
    }
}

pub type DirIter = Box<Iterator<Item = Directory>>;

pub fn find_in(query: &str, origin: &Path) -> Result<DirIter, Error> {
    let query = String::from(query);
    let base_dir = PathBuf::from(origin);

    Ok(
        Box::new(
            try!(read_dir(base_dir)).filter_map(move |dir|{
                let dir = dir.unwrap();
                let dir_name = String::from(dir.file_name().to_str().unwrap());
                if dir.file_type().unwrap().is_dir() && dir_name.starts_with(&query) {
                    Some(Directory {
                        path: PathBuf::from(dir.path()),
                        name: dir_name
                    })
                }
                else {
                    None
                }
            })
        )
    )
}
