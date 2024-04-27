use crate::connector::error::ConnectorError;
use crate::connector::kraken::authentication::{
    get_api_sign, get_nonce, API_KEY_HEADER, API_SIGN_HEADER,
};
use crate::connector::kraken::settings::{KRAKEN_API_BASE_URL, KRAKEN_API_KEY, KRAKEN_API_SECRET};
use reqwest::Url;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;

#[derive(Debug, Deserialize)]
pub struct AccountBalanceResponse {
    #[allow(dead_code)]
    error: Vec<String>,

    #[allow(dead_code)]
    result: Option<AccountBalance>,
}

pub type AccountBalance = HashMap<String, Decimal>;

#[derive(Serialize)]
struct AccountBalanceRequest {
    nonce: u128,
}

pub async fn get_account_balance() -> Result<AccountBalance, ConnectorError> {
    // auth
    let nonce = get_nonce();
    let data = AccountBalanceRequest { nonce };
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

    let result = res.json::<AccountBalanceResponse>().await?;
    match result.result {
        Some(result) => Ok(result),
        None => Err(ConnectorError::DataError(result.error)),
    }
}
