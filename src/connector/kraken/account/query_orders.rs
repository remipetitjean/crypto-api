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
pub struct QueryOrdersResponse {
    #[allow(dead_code)]
    error: Vec<String>,

    #[allow(dead_code)]
    result: Option<OrderMap>,
}

pub type TradeId = String;

pub type OrderMap = HashMap<TradeId, QueryOrder>;

#[derive(Debug, Deserialize)]
pub struct QueryOrder {
    #[allow(dead_code)]
    refid: Option<String>,
    #[allow(dead_code)]
    userref: Option<i32>,
    #[allow(dead_code)]
    status: String,
    #[allow(dead_code)]
    opentm: Decimal,
    #[allow(dead_code)]
    closetm: Option<Decimal>,
    #[allow(dead_code)]
    starttm: Decimal,
    #[allow(dead_code)]
    expiretm: Decimal,
    #[allow(dead_code)]
    descr: OrderDescr,
    #[allow(dead_code)]
    vol: Decimal,
    #[allow(dead_code)]
    vol_exec: Decimal,
    #[allow(dead_code)]
    cost: Decimal,
    #[allow(dead_code)]
    fee: Decimal,
    #[allow(dead_code)]
    price: Decimal,
    #[allow(dead_code)]
    stopprice: Decimal,
    #[allow(dead_code)]
    limitprice: Decimal,
    #[allow(dead_code)]
    trigger: Option<String>,
    #[allow(dead_code)]
    misc: String,
    #[allow(dead_code)]
    oflags: String,
    #[allow(dead_code)]
    trades: Option<Vec<TradeId>>,
}

#[derive(Debug, Deserialize)]
pub struct OrderDescr {
    #[allow(dead_code)]
    pair: String,
    #[allow(dead_code)]
    r#type: String,
    #[allow(dead_code)]
    ordertype: String,
    #[allow(dead_code)]
    price: Decimal,
    #[allow(dead_code)]
    price2: Decimal,
    #[allow(dead_code)]
    leverage: String,
    #[allow(dead_code)]
    order: String,
    #[allow(dead_code)]
    close: String,
}

#[derive(Serialize)]
struct QueryOrdersRequest {
    nonce: u128,
    txid: String,
    trades: bool,
    userref: Option<i32>,
    consolidate_taker: Option<bool>,
}

pub async fn get_query_orders(
    txid: String,
    trades: bool,
    userref: Option<i32>,
    consolidate_taker: Option<bool>,
) -> Result<OrderMap, ConnectorError> {
    // auth
    let nonce = get_nonce();
    let data = QueryOrdersRequest {
        nonce,
        txid,
        trades,
        userref,
        consolidate_taker,
    };
    let path = "/0/private/QueryOrders";
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

    let result = res.json::<QueryOrdersResponse>().await?;
    match result.result {
        Some(result) => Ok(result),
        None => Err(ConnectorError::DataError),
    }
}
