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

#[derive(Debug, Deserialize, Clone)]
pub struct ApiResponse<Res> {
    error: Vec<String>,
    result: Option<Res>,
}

pub async fn send_public_request<Req, Res>(
    endpoint: String,
    request_data: Req,
) -> Result<Res, ConnectorError>
where
    Req: Serialize,
    Res: for<'a> Deserialize<'a>,
{
    let url_str = format!("{KRAKEN_API_BASE_URL}/0/public/{endpoint}");
    let url = Url::parse(&url_str).unwrap();

    let client = reqwest::Client::new();
    let res = client.post(url).form(&request_data).send().await?;

    let result = res.json::<ApiResponse<Res>>().await?;
    match result.result {
        Some(result) => Ok(result),
        None => Err(ConnectorError::DataError(result.error)),
    }
}
