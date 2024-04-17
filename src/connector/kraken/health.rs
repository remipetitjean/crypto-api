use super::super::error::ConnectorError;
use super::settings::KRAKEN_API_BASE_URL;
use reqwest::get;
use serde::Deserialize;

#[derive(Deserialize)]
struct ServerTimeResponse {
    #[allow(dead_code)]
    error: Vec<String>,

    #[allow(dead_code)]
    result: ServerTime,
}

#[derive(Deserialize)]
pub struct ServerTime {
    pub unixtime: u64,
    pub rfc1123: String,
}

#[allow(dead_code)]
pub async fn get_server_time() -> Result<ServerTime, ConnectorError> {
    let url = format!("{KRAKEN_API_BASE_URL}/0/public/Time");
    let server_time = get(url).await?.json::<ServerTimeResponse>().await?.result;
    Ok(server_time)
}
