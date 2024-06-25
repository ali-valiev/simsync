mod fetch;
mod parse;
mod traverse_and_save;
mod types;

use anyhow::Result;
use fetch::get_remote_html;
use parse::parse_to_json;
use traverse_and_save::traverse_and_save;

use log::{error, info};

pub async fn run(host: &str, port: u16) -> Result<()> {
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

    match traverse_and_save(&json) {
        Ok(file_count) => {
            info!("Succesfully traversed {file_count} files and saved in files.json");
        }
        Err(e) => {
            error!("Could not traverse adn save files: {e}");
        }
    };

    Ok(())
}
