use super::super::error::ConnectorError;
use super::settings::KRAKEN_API_BASE_URL;
use reqwest::Url;
use serde::Deserialize;
use std::collections::HashMap;
use std::str;

// https://support.kraken.com/hc/en-us/articles/360001491786-API-error-messages

#[derive(Debug, Deserialize)]
pub struct AccountBalanceResponse {
    error: Vec<String>,
    result: Option<HashMap<String, f64>>,
}

pub async fn get_account_balance() -> Result<String, ConnectorError> {
    let url_str = format!("{KRAKEN_API_BASE_URL}/0/private/Balance");
    let url = Url::parse(&url_str).expect("could not parce URL");
    println!("{:?}", url);

    // let client = reqwest::Client::new();
    // let res = client.post(url).send().await?;

    // res.error_for_status_ref()?;

    // let text = res.text().await?;

    Ok("text".to_string())
}
