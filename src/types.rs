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
    files: Vec<File>,
    file_count: usize,
}

impl Output {
    pub fn new(sync_date: String, files: Vec<File>, file_count: usize) -> Self {
        Self { sync_date, files, file_count}
    }
}
