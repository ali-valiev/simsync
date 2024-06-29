use anyhow::Result;
use log::info;
use std::fs;

use crate::types::File;

pub async fn sync(
    old_files: Option<Vec<File>>,
    new_files: Vec<File>,
    host: String,
    port: u16,
    dir: &str,
) -> Result<()> {
    info!("searching for file index in  {}", dir);

    if old_files.is_none() {
        for file in new_files {
            download_file(file, &host, port, dir).await?;
        }
    } else {
        for old_file in old_files.clone().unwrap() {
            if new_files.contains(&old_file) {
                info!(
                    "file '{name}' already synced, skipping.",
                    name = old_file.get_name()
                );
            } else {
                info!(
                    "Could not find {name} in remote directory, removing it...",
                    name = old_file.get_name()
                );
                let path = format!("{dir}{name}", name = old_file.get_name());
                fs::remove_file(path)?;
            }
        }

        for new_file in new_files {
            if old_files.clone().unwrap().contains(&new_file) {
                info!(
                    "Skipping fetching {name}, it is already synced",
                    name = new_file.get_name()
                );
            } else {
                download_file(new_file, &host, port, dir).await?;
            }
        }
    }

    Ok(())
}

async fn download_file(file: File, host: &str, port: u16, dir: &str) -> Result<()> {
    let url = format!("http://{host}:{port}/{path}", path = file.get_path());
    let file_data = reqwest::get(&url).await?;
    let file_path = file.get_name();
    let path = format!("{}/{}", dir, file_path);

    if !fs::metadata(&path).is_ok() {
        fs::write(&path, file_data.bytes().await?)?;
        info!("Successfully synced {}", file_path);
    } else {
        info!("File '{}' exists, skipping...", file_path);
    }

    Ok(())
}
