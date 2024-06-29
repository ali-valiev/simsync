mod fetch;
mod get_file_index;
mod parse;
mod sync;
mod traverse_and_save;
mod types;

use fetch::get_remote_html;
use get_file_index::{get_file_index, get_old_file_index};
use parse::parse_to_json;
use sync::sync;
use traverse_and_save::traverse_and_save;

use crate::types::File;

use log::{error, info};
use std::env;
use std::error::Error;

fn get_port(count: String) -> Result<u16, String> {
    match count.parse::<u16>() {
        Ok(count) => Ok(count),
        Err(e) => Err(e.to_string()),
    }
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let file_index_name = "file_index.json";

    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1].to_lowercase() == "help" {
        println!("SimSync v1.0.0");
        println!("Usage: simsync [Host] [Port] [Local directory]");
        println!("Host - Ipv4 address of the python http server running in the remote directory");
        println!("Port - Port that the python http server runs on");
        println!("Local Directory - the directory where the contents of the remote directory will be downloaded to");
        println!("Keep in mind that SimSync won't copy other directories within the remote directory itself");
        println!("Created by Ali Valiev, June 2024");
        return Ok(());
    } else if args.len() < 4 {
        return Err("Invalid argument count. Run with 'help' flag to get help".into());
    }
    let host = &args[1];
    let port = get_port(args[2].clone())?;
    let dir = &args[3];

    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let old_files: Option<Vec<File>>;
    match get_old_file_index(file_index_name, dir) {
        Ok(files) => {
            old_files = files;
        }
        Err(e) => {
            error!("Could not get old file index: {e}");
            return Err(e.into());
        }
    }

    let res = match get_remote_html(host, port).await {
        Ok(response) => {
            info!("Got the response!");
            response
        }
        Err(e) => {
            error!("Could not get response from the server: {e}");
            return Err(e.into());
        }
    };

    let res_text = match res.text().await {
        Ok(text) => {
            info!("Got the data out from response!");
            text
        }
        Err(e) => {
            error!("Could not get the data out of response: {e}");
            return Err(e.into());
        }
    };

    let json = match parse_to_json(&res_text) {
        Ok(json) => {
            info!("Successfully parsed HTML data to Json");
            json
        }
        Err(e) => {
            error!("Could not parse data to Json: {e}");
            return Err(e.into());
        }
    };

    match traverse_and_save(&file_index_name, &json, &dir) {
        Ok(file_count) => {
            info!("Succesfully traversed {file_count} files and saved in {file_index_name}");
        }
        Err(e) => {
            error!("Could not traverse and save files: {e}");
            return Err(e.into());
        }
    };

    let file_index = match get_file_index(file_index_name, dir) {
        Ok(files) => files,
        Err(e) => {
            error!("Could not get new file index: {e}");
            return Err(e.into());
        }
    };

    match sync(old_files, file_index, host.to_string(), port, dir).await {
        Ok(_) => {
            info!("Succesfully synced all files from {host}:{port}")
        }
        Err(e) => {
            error!("Could not sync files: {e}");
            return Err(e.into());
        }
    }

    Ok(())
}
