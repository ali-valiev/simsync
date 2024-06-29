use crate::types::{File, Output};
use anyhow::Result;
use std::fs;

pub fn get_file_index(file_index_name: &str, dir: &str) -> Result<Vec<File>> {
    let path = format!("{dir}{file_index_name}");
    let files = fs::read_to_string(path)?;
    let files_json = serde_json::from_str::<Output>(&files)?;
    let files = files_json.get_files();
    Ok(files)
}

pub fn get_old_file_index(file_index_name: &str, dir: &str) -> Result<Option<Vec<File>>> {
    let mut old_files: Option<Vec<File>> = None;
    let file_index_path = format!("{dir}/{file_index_name}");
    if fs::metadata(&file_index_path).is_ok() {
        let old_file_index = fs::read_to_string(file_index_path)?;
        let old_file_index_json = serde_json::from_str::<Output>(&old_file_index)?;
        old_files = Some(old_file_index_json.get_files());
    }
    Ok(old_files)
}
