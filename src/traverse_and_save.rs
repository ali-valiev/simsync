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
            dbg!(&file["children"][0]["children"][0]["as"]);
            // if let (name, path) = (
                // file["children"][0]["children"][0]["as"].as_str().unwrap(),
                // file["children"][0]["attributes"]["href"].as_str().unwrap(),
            // ) {
                // files.push(File::new(name.to_string(), path.to_string()));
            // }
        }
    }

    dbg!(&files);

    let date: DateTime<Utc> = Utc::now();
    let file_count = files.len();
    let output = Output::new(date.to_string(), files, file_count);
    let output_json = serde_json::to_string_pretty(&output)?;
    fs::write("files.json", output_json)?;
    // let json_to_write = format!(
    //     r#"{{ "lastSynced": "{date}", "files": [{data}]}}"#,
    //     date = date.to_rfc3339(),
    //     data = file_vec.join(", ")
    // );
    //
    // fs::write("files.json", json_to_write)?;

    Ok(file_count)
}
