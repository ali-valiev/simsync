use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    name: String,
    path: String,
}

impl Clone for File {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            path: self.path.clone(),
        }
    }
}

impl File {
    pub fn new(path: String, name: String) -> Self {
        Self { path, name }
    }
    pub fn get_path(&self) -> &str {
        &self.path
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    sync_date: String,
    file_count: usize,
    files: Vec<File>,
}

impl Output {
    pub fn new(sync_date: String, file_count: usize, files: Vec<File>) -> Self {
        Self {
            sync_date,
            files,
            file_count,
        }
    }

    pub fn get_files(self) -> Vec<crate::types::File> {
        self.files
    }
}
