use std::path::PathBuf;

use serde::Serialize;

struct FileList {
    files: Vec<CustomFile>,
}

impl FileList {
    fn new() -> Self {
        FileList { files: Vec::new() }
    }

    fn add_file(&mut self, file: CustomFile) -> &Self {
        self.files.push(file);
        self
    }
}

#[derive(Debug, Serialize)]
pub struct CustomFile {
    path: PathBuf,
    name: String,
    extension: String,
    parent: PathBuf,
    size: u64,
    is_file: bool,
    is_dir: bool,
    children: Vec<CustomFile>,
}

impl CustomFile {
    pub fn new(path: PathBuf) -> Result<Self, std::io::Error> {
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let extension = path.extension().unwrap().to_str().unwrap().to_string();
        let parent = path.parent().unwrap().to_path_buf();
        let size = std::fs::metadata(&path).unwrap().len();
        let is_file = std::fs::metadata(&path).unwrap().is_file();
        let is_dir = std::fs::metadata(&path).unwrap().is_dir();

        Ok(CustomFile {
            path,
            name,
            size,
            extension,
            parent,
            is_dir,
            is_file,
            children: vec![],
        })
    }
}
