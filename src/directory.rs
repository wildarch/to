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

pub fn find_in(query: &Path, origin: &Path) -> Result<DirIter, Error> {
    let mut base_dir = PathBuf::from(origin);
    // We have to adjust the base directory if needed
    let mut components = query.components();
    let num_comp = query.components().count();

    if num_comp > 0 {
        for comp in components.by_ref().take(num_comp-1) {
            let string = comp.as_os_str().to_str().expect("Invalid character in os str");
            base_dir.push(string);
        }
    }

    let query = match components.next() {
        Some(comp) => comp.as_os_str().to_str().unwrap(),
        None => ""
    };

    let query = String::from(query);

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
