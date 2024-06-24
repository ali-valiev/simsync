use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    file_name: String,
}

impl File {
    pub fn new(file_name: String) -> File {
        File { file_name }
    }
}
