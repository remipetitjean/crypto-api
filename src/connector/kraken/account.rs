use super::super::error::ConnectorError;
use super::authentication::{get_api_sign, get_nonce, API_KEY_HEADER, API_SIGN_HEADER};
use super::settings::{KRAKEN_API_BASE_URL, KRAKEN_API_KEY, KRAKEN_API_SECRET};
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

pub async fn get_account_balance() -> Result<AccountBalanceResponse, ConnectorError> {
    // auth
    let nonce = get_nonce();
    let params = [("nonce", nonce.to_string())];
    let url_str = format!("{KRAKEN_API_BASE_URL}/0/private/Balance");
    let url = Url::parse_with_params(&url_str, &params).unwrap();
    let sig = get_api_sign(url.to_owned(), nonce, KRAKEN_API_SECRET.to_string());

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header(API_KEY_HEADER, KRAKEN_API_KEY)
        .header(API_SIGN_HEADER, sig)
        .send()
        .await?;
    let _status = res.status();
    let account_balance = res.json::<AccountBalanceResponse>().await?;

    Ok(account_balance)
    // res.error_for_status_ref()?;

    // let text = res.text().await?;
}
