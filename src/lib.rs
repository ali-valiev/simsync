mod download;
mod fetch;
mod parse;
mod traverse_and_save;
mod types;

use download::download;
use fetch::get_remote_html;
use parse::parse_to_json;
use traverse_and_save::traverse_and_save;

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
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args[1].to_lowercase() == "help" {
        println!("this is a help message");
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

    match traverse_and_save(&json, &dir) {
        Ok(file_count) => {
            info!("Succesfully traversed {file_count} files and saved in files.json");
        }
        Err(e) => {
            error!("Could not traverse adn save files: {e}");
        }
    };

    match download("files.json", host.to_string(), port, dir.to_string()).await {
        Ok(_) => info!("Succesfully synced all files from {host}:{port}"),
        Err(e) => error!("Could not sync all files: {e}"),
    }

    Ok(())
}
