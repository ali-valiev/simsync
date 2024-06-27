use anyhow::Result;
use log::info;
use serde_json;
use std::fs;

use crate::types::Output;

pub async fn download(files: &str, host: &str, port: u16) -> Result<()> {
    info!("Retrieving files to download from {}", &files);
    let files = fs::read_to_string(files)?;
    let files_json = serde_json::from_str::<Output>(&files)?;
    let files = files_json.get_files();

    for file in files {
        let url = format!("http://{host}:{port}/{path}", path = file.get_path());
        let file_data = reqwest::get(&url).await?;
        let file_path = file.get_name();
        if !fs::metadata(file_path).is_ok() {
            fs::write(file.get_name(), file_data.text().await?)?;
            info!("Succesfully synced {file_path}");
        } else {
            info!("file '{file_path}' exists, Skipping...");
        }
    }

    Ok(())
}
