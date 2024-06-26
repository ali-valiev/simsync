use crate::types::File;
use crate::types::Output;

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::fs;

pub fn traverse_and_save(data: &str) -> Result<usize> {
    let data: Value = serde_json::from_str(data).expect("error while parsing the json");

    let mut files: Vec<File> = Vec::new();

    if let Some(files_array) =
        data["children"][0]["children"][1]["children"][2]["children"].as_array()
    {
        dbg!(&files_array);
        for file in files_array {
            if let (Some(name), Some(path)) = (
                &file["children"][0]["attributes"]["href"].as_str(),
                &file["children"][0]["children"][0].as_str(),
            ) {
                files.push(File::new(
                    name.trim_matches('"').to_string(),
                    path.trim_matches('"').to_string(),
                ));
            }
        }
    }

    let date: DateTime<Utc> = Utc::now();
    let file_count = files.len();
    let output = Output::new(date.to_string(), file_count, files);
    let output_json = serde_json::to_string_pretty(&output)?;
    fs::write("files.json", output_json)?;

    Ok(file_count)
}
