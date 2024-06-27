use anyhow::Result;
use log::info;
use serde_json;
use std::fs;

use crate::types::{File, Output};

pub async fn download(files: &str, host: String, port: u16, dir: String) -> Result<()> {
    info!("Retrieving files to download from {}", files);
    let path = format!("{dir}{files}");
    let files = fs::read_to_string(path)?;
    info!("files pass");
    let files_json = serde_json::from_str::<Output>(&files)?;
    info!("files_json pass");
    let files = files_json.get_files();
    info!("files pass");

    for file in files {
        file_shit(file, &host, port, &dir).await?;
    }

    Ok(())
}

async fn file_shit(file: File, host: &str, port: u16, dir: &str) -> Result<()> {
    let url = format!(
        "http://{host}:{port}/{path}",
        host = host,
        port = port,
        path = file.get_path()
    );
    let file_data = reqwest::get(&url).await?;
    info!("file_data pass");
    let file_path = file.get_name();
    info!("file_path pass");
    let path = format!("{}/{}", dir, file_path);
    info!("path pass");

    if !fs::metadata(&path).is_ok() {
        fs::write(&path, file_data.bytes().await?)?;
        info!("gotta be done");
        info!("Successfully synced {}", file_path);
    } else {
        info!("File '{}' exists, skipping...", file_path);
    }

    Ok(())
}
