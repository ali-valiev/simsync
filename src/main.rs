use anyhow::Result;
use simsync::run;

#[tokio::main]
async fn main() -> Result<(), String> {
    match run().await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("ERROR: {e}")),
    }
}
