use crate::types::File;
use crate::types::Output;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::fs;

// Most of the logic of traversing is based on structure of the html file parsed into json
// file names and paths of the remote files are nested within the json, so if you want to take a
// look at it, feel free to review structure of the html parsed to json
//
// all the data that has to be saved in the index file are serialized from the types::Output struct
//
// i've also placed some usefull info like last sync time in the index file
pub fn traverse_and_save(file_index_name: &str, data: &str, dir: &str) -> Result<usize> {
    let data: Value = serde_json::from_str(data).expect("error while parsing the json");

    let mut files: Vec<File> = Vec::new();

    if let Some(files_array) =
        data["children"][0]["children"][1]["children"][2]["children"].as_array()
    {
        for file in files_array {
            if let (Some(path), Some(name)) = (
                &file["children"][0]["attributes"]["href"].as_str(),
                &file["children"][0]["children"][0].as_str(),
            ) {
                if !name.ends_with("/") && !path.ends_with("/") {
                    files.push(File::new(
                        path.trim_matches('"').to_string(),
                        name.trim_matches('"').to_string(),
                    ));
                }
            }
        }
    }

    let date: DateTime<Utc> = Utc::now();
    let file_count = files.len();
    let output = Output::new(date.to_string(), file_count, files);
    let output_json = serde_json::to_string_pretty(&output)?;
    let dir = format!("{dir}/{file_index_name}");
    fs::write(&dir, output_json)?;

    Ok(file_count)
}
