use crate::types::{File, Output};
use anyhow::Result;
use std::fs;

// This fucntion read the index file and return it as a vector of types::File
pub fn get_new_file_index(file_index_name: &str, dir: &str) -> Result<Vec<File>> {
    let path = format!("{dir}/{file_index_name}");
    let files = fs::read_to_string(path)?;
    let files = serde_json::from_str::<Output>(&files)?.get_files();

    Ok(files)
}

// To get the current file index of the local directory we need to read all the files from the
// directory itself insted of the index file
// Cuz if an error occures while downloading the files from the remote directory sunc porcess will
// stop, and not all the files will be synced
// But no matter the outcome of the sync() the remote file index will be saved as the local index,
// so we wont't know if all the files are synced or not
pub fn get_current_file_index(dir: &str) -> Result<Option<Vec<String>>> {
    let dir_files = fs::read_dir(dir);
    match dir_files {
        Ok(files) => {
            let mut current_files: Vec<String> = vec![];
            for entry in files {
                let file_name: String = entry?.file_name().to_string_lossy().into_owned();
                current_files.push(file_name);
            }
            Ok(Some(current_files))
        }
        Err(e) => {
            return Err(e.into());
        }
    }
}
