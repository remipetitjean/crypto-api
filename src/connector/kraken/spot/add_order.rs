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

#[derive(Serialize)]
struct AddOrdersRequest {
    nonce: u128,
    pair: String,
    ordertype: String,
    r#type: String,
    volume: Decimal,
    userref: Option<i32>,
    displayvol: Option<String>,
    price: Option<Decimal>,
    price2: Option<Decimal>,
    trigger: Option<String>,
    leverage: Option<String>,
    reduceonly: Option<bool>,
    stptype: Option<String>,
    oflags: Option<String>,
    timeinforce: Option<String>,
    startm: Option<Decimal>,
    expiretm: Option<Decimal>,
    deadline: Option<String>,
    validate: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct AddOrdersResponse {
    #[allow(dead_code)]
    error: Vec<String>,

    #[allow(dead_code)]
    result: Option<AddedOrder>,
}

pub type TradeId = String;

#[derive(Debug, Deserialize)]
pub struct AddedOrder {
    descr: OrderDescr,
    txid: Vec<TradeId>,
}

#[derive(Debug, Deserialize)]
pub struct OrderDescr {
    #[allow(dead_code)]
    order: String,
}

pub async fn add_order(
    pair: String,
    ordertype: String,
    r#type: String,
    volume: Decimal,
    userref: Option<i32>,
    displayvol: Option<String>,
    price: Option<Decimal>,
    price2: Option<Decimal>,
    trigger: Option<String>,
    leverage: Option<String>,
    reduceonly: Option<bool>,
    stptype: Option<String>,
    oflags: Option<String>,
    timeinforce: Option<String>,
    startm: Option<Decimal>,
    expiretm: Option<Decimal>,
    deadline: Option<String>,
    validate: Option<bool>,
) -> Result<AddedOrder, ConnectorError> {
    // auth
    let nonce = get_nonce();
    let data = AddOrdersRequest {
        nonce,
        pair,
        ordertype,
        r#type,
        volume,
        userref,
        displayvol,
        price,
        price2,
        trigger,
        leverage,
        reduceonly,
        stptype,
        oflags,
        timeinforce,
        startm,
        expiretm,
        deadline,
        validate,
    };
    let path = "/0/private/AddOrder";
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

    // let result_str = res.text().await?;
    // println!("result = {}", result_str);
    // Err(ConnectorError::DataError)

    let result = res.json::<AddOrdersResponse>().await?;
    match result.result {
        Some(result) => Ok(result),
        None => Err(ConnectorError::DataError(result.error)),
    }
}
