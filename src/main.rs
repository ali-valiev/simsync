use anyhow::Result;
use simsync::run;

// I've split up the crate into binary and library crates, so the logic of the app is in lib.rs

#[tokio::main]
async fn main() -> Result<(), String> {
    match run().await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("ERROR: {e}")),
    }
}
