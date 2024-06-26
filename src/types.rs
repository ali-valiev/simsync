use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    name: String,
    path: String,
}

impl File {
    pub fn new(name: String, path: String) -> Self {
        Self { name, path }
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
}
