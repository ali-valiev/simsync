use anyhow::Result;
use simsync::run;

#[tokio::main]
async fn main() -> Result<()> {
    run("127.0.0.1", 8000).await?;
    Ok(())
}
