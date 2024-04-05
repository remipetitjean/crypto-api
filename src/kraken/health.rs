use super::settings::KRAKEN_API_URL;
use reqwest::{get, Result};

struct ServerTime {
    unixtime: u64,
    rfc1123: &str,
}

pub async fn get_server_time() -> Result<()> {
    let url = format!("{KRAKEN_API_URL}/public/Time");
    let resp = get(url).await?.text().await?;
    println!("{resp}");
    Ok(())
}
