use crate::types::File;
use chrono::{DateTime, Utc};
use serde_json::Value;
use std::fs;
use anyhow::Result;

pub fn traverse_and_save(data: &str) -> Result<()> {
    let data: Value = serde_json::from_str(data).expect("error while parsing the json");

    let mut file_vec: Vec<String> = Vec::new();

    if let Some(files) = data["children"][0]["children"][1]["children"][2]["children"].as_array() {
        for file in files {
            if let Some(file_name) = file["children"][0]["children"][0].as_str() {
                file_vec.push(file_name.to_string());
            }
        }
    }

    let file_vec: Vec<String> = file_vec
        .iter()
        .map(|file_name| serde_json::to_string(&File::new(file_name.to_string())).unwrap())
        .collect();

    let date: DateTime<Utc> = Utc::now();
    let json_to_write = format!(
        r#"{{ "lastSynced": {date}, "files": [{data}]}}"#,
        date = date.to_rfc3339().to_string(),
        data = file_vec.join(", ")
    );

    fs::write("files.json", json_to_write)?;

    Ok(())
}
