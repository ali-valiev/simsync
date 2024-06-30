use anyhow::Result;
use log::info;
use std::fs;

use crate::types::File;

pub async fn sync(
    current_files: Option<Vec<String>>,
    new_files: Vec<File>,
    host: String,
    port: u16,
    dir: &str,
) -> Result<(usize, String)> {
    // savung the start time of sync process
    let start = chrono::Utc::now();

    info!("searching for file index in {}", dir);

    // getting the names of new files from new index file to compare with existing ones
    let mut new_file_names: Vec<String> = vec![];
    for new_file in &new_files {
        new_file_names.push(new_file.get_name().to_string());
    }

    // If no current files exist, it will simply loop through th efiles one by one and download
    // them
    if current_files.is_none() {
        for file in &new_files {
            download_file(file.clone(), &host, port, dir).await?;
        }
    } else {
        // if not, it will loop through existing files and remove them if they no longer exist in
        // the remote directory. Or if it is already in the remote directory it will the file in
        // peace
        for current_file in current_files.clone().unwrap() {
            if new_file_names.contains(&current_file) {
                info!("file '{current_file}' already synced, skipping.",);
            } else {
                info!("Could not find {current_file} in remote directory, removing it...",);
                let path = format!("{dir}{current_file}");
                fs::remove_file(path)?;
            }
        }

        // Then it will loop through the new files one by one, and check if they exist
        // if a file already exists in the local directory it will not download it again
        // else it will download it
        for new_file in &new_files {
            if current_files
                .clone()
                .unwrap()
                .contains(&new_file.get_name().to_string())
            {
                info!(
                    "Skipping fetching {new_file_name}, it is already synced",
                    new_file_name = new_file.get_name().to_string()
                );
            } else {
                download_file(new_file.clone(), &host, port, dir).await?;
            }
        }
    }

    // At last, getting the current time to calculate sync duration
    let end = chrono::Utc::now();
    let time_elapsed = end.signed_duration_since(start);

    Ok((new_files.len(), format_duration(time_elapsed.num_seconds())))
}

// Main logic of downlading the file itself
async fn download_file(file: File, host: &str, port: u16, dir: &str) -> Result<()> {
    let url = format!("http://{host}:{port}/{path}", path = file.get_path());
    info!("Downloading {file_name}...", file_name = file.get_name());
    let file_data = reqwest::get(&url).await?;
    let path = format!("{dir}/{file_name}", file_name = file.get_name());

    if !fs::metadata(&path).is_ok() {
        fs::write(&path, file_data.bytes().await?)?;
        info!("Successfully synced {}", file.get_name());
    } else {
        info!("File '{}' exists, skipping...", file.get_name());
    }

    Ok(())
}

// Simple formatter of duration that formats seconds to minutes and hours if possible
fn format_duration(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    let mut duration = String::new();

    if hours > 0 {
        duration.push_str(&format!("{}h", hours));
    }

    if minutes > 0 {
        duration.push_str(&format!("{}m", minutes));
    }

    duration.push_str(&format!("{}s", seconds));
    duration.trim().to_string()
}
