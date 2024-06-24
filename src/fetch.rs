use reqwest::Client;

pub async fn get_remote_html(host: &str, port: u16) -> Result<reqwest::Response, reqwest::Error> {
    let url = format!("http://{host}:{port}");
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("simparser/1.0")
        .build()?;

    let res = client.get(&url).send().await?;
    Ok(res)
}
