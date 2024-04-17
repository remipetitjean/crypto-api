use super::super::error::ConnectorError;
use super::authentication::{get_api_sign, get_nonce, API_KEY_HEADER, API_SIGN_HEADER};
use super::settings::{KRAKEN_API_BASE_URL, KRAKEN_API_KEY, KRAKEN_API_SECRET};
use reqwest::Url;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;

// https://support.kraken.com/hc/en-us/articles/360001491786-API-error-messages

#[derive(Debug, Deserialize)]
pub struct AccountBalanceResponse {
    error: Vec<String>,
    result: Option<HashMap<String, Decimal>>,
}

#[derive(Serialize)]
struct EmptyData {
    nonce: u128,
}

pub async fn get_account_balance() -> Result<AccountBalanceResponse, ConnectorError> {
    // auth
    let nonce = get_nonce();
    println!("{nonce}");
    let data = EmptyData { nonce };
    let path = "/0/private/Balance";
    let sig = get_api_sign(
        path.to_string(),
        nonce,
        &data,
        KRAKEN_API_SECRET.to_string(),
    );

    let url_str = format!("{KRAKEN_API_BASE_URL}{path}");
    let url = Url::parse(&url_str).unwrap();

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header(API_KEY_HEADER, KRAKEN_API_KEY)
        .header(API_SIGN_HEADER, sig)
        .form(&data)
        .send()
        .await?;

    let _status = res.status();
    let account_balance = res.json::<AccountBalanceResponse>().await?;

    Ok(account_balance)
}
