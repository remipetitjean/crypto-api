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
pub struct ClosedOrdersResponse {
    #[allow(dead_code)]
    error: Vec<String>,

    #[allow(dead_code)]
    result: Option<TypedOrders>,
}

pub type TradeId = String;

#[derive(Debug, Deserialize)]
struct TypedOrders {
    closed: Option<OrderMap>,
}

pub type OrderMap = HashMap<TradeId, ClosedOrder>;

#[derive(Debug, Deserialize)]
pub struct ClosedOrder {
    #[allow(dead_code)]
    refid: Option<String>,
    #[allow(dead_code)]
    userref: Option<i32>,
    #[allow(dead_code)]
    status: String,
    #[allow(dead_code)]
    opentm: Decimal,
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
    #[allow(dead_code)]
    closetm: Decimal,
    #[allow(dead_code)]
    reason: Option<String>,
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
struct ClosedOrdersRequest {
    nonce: u128,
    trades: bool,
    userref: Option<i32>,
    start: Option<u128>,
    end: Option<u128>,
    ofs: Option<i32>,
    closetime: Option<String>,
    consolidate_taker: Option<bool>,
}

pub async fn get_closed_orders(
    trades: bool,
    userref: Option<i32>,
    start: Option<u128>,
    end: Option<u128>,
    ofs: Option<i32>,
    closetime: Option<String>,
    consolidate_taker: Option<bool>,
) -> Result<OrderMap, ConnectorError> {
    // auth
    let nonce = get_nonce();
    let data = ClosedOrdersRequest {
        nonce,
        trades,
        userref,
        start,
        end,
        ofs,
        closetime,
        consolidate_taker,
    };
    let path = "/0/private/ClosedOrders";
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

    let result = res.json::<ClosedOrdersResponse>().await?;
    match result.result {
        Some(result) => match result.closed {
            Some(opens) => Ok(opens),
            None => Ok(HashMap::new()),
        },
        None => Err(ConnectorError::DataError(result.error)),
    }
}
