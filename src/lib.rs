mod fetch;
mod get_file_index;
mod parse;
mod sync;
mod traverse_and_save;
mod types;

use fetch::get_remote_html;
use get_file_index::{get_current_file_index, get_new_file_index};
use parse::parse_to_json;
use sync::sync;
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

    // Reading the local directory to check if there are any files
    // There could be all the files that are in the remote directory that has to be synchronized
    // In that case no synchronization is needeed
    // But sometimes there are files in local directory that are not in remote directory, in that
    // case that file has to be removed becasue SimSync is OneWay synchronization tool
    let current_files: Option<Vec<String>>;
    match get_current_file_index(dir) {
        Ok(files) => {
            current_files = files;
        }
        Err(e) => {
            error!("Could not get old file index: {e}");
            return Err(e.into());
        }
    }

    // Response is an html file that remote python server provides
    // it is in its raw form
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

    //To get the html data out of the response we need to get its text()
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

    //Then in order to parse it effectively we need to convert it to json
    //I choose json because of mine familiarity with it and it fitted the needs perfectly
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

    // now that the all data is on our hands wee need to parse it
    // after proper parsing we need to save it in order to have access to the files data right away
    match traverse_and_save(&file_index_name, &json, &dir) {
        Ok(file_count) => {
            info!("Succesfully traversed {file_count} files and saved in {file_index_name}");
        }
        Err(e) => {
            error!("Could not traverse and save files: {e}");
            return Err(e.into());
        }
    };

    // After parsing and saving the file index we need to read it from the index file
    let file_index = match get_new_file_index(file_index_name, dir) {
        Ok(files) => files,
        Err(e) => {
            error!("Could not get new file index: {e}");
            return Err(e.into());
        }
    };

    // last step is to check the synchronization
    // we pass current file index, and the up to date file index that we just got from the remote
    // server to the sync() function for it to handle all the files, local and remote
    match sync(current_files, file_index, host.to_string(), port, dir).await {
        Ok((file_count, time_elapsed)) => {
            info!("Succesfully synced {file_count} files from {host}:{port} in {time_elapsed}");
        }
        Err(e) => {
            error!("Could not sync files: {e}");
            return Err(e.into());
        }
    }

    Ok(())
}
