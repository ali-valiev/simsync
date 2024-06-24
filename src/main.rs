mod fetch;
mod parse;
mod traverse_and_save;
mod types;

use anyhow::Result;
use fetch::get_remote_html;
use parse::parse_to_json;
use traverse_and_save::traverse_and_save;

use log::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    match get_remote_html("127.0.0.1", 8000).await {
        Ok(res) => {
            info!("Got the response");
            match parse_to_json(&res.text().await?) {
                Ok(json) => {
                    info!("Got the parsing to json");
                    
                    info!("Starting to save files...");
                    if let Err(e) = traverse_and_save(&json) {
                        error!("Error while traversing and saving to file: {e}");
                    }
                }
                Err(e) => {
                    error!("Error while parsing request to json: {e}");
                }
            }
        }
        Err(e) => {
            error!("Error while getting data: {e}");
        }
    };


    Ok(())
}
