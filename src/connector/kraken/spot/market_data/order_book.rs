use crate::connector::error::ConnectorError;
use crate::connector::kraken::settings::KRAKEN_API_BASE_URL;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;

use polars::prelude::*;
use rust_decimal::Decimal;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct OrderBookRequest {
    pair: String,
    count: Option<i16>,
}
#[derive(Debug, Deserialize)]
pub struct OrderBookResponse {
    #[allow(dead_code)]
    error: Vec<String>,

    #[allow(dead_code)]
    result: Option<HashMap<String, OrderBookBidAsk>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OrderBookBidAsk {
    pub asks: Vec<(Decimal, Decimal, f64)>,
    pub bids: Vec<(Decimal, Decimal, f64)>,
}

pub async fn get_order_book(
    pair: String,
    count: Option<i16>,
) -> Result<OrderBookBidAsk, ConnectorError> {
    // auth
    let data = OrderBookRequest {
        pair: pair.clone(),
        count,
    };
    let path = "/0/public/Depth";

    let url_str = format!("{KRAKEN_API_BASE_URL}{path}");
    let url = Url::parse(&url_str).unwrap();

    let client = reqwest::Client::new();
    let res = client.post(url).form(&data).send().await?;

    let result = res.json::<OrderBookResponse>().await?;
    match result.result {
        Some(result) => match result.get(&pair) {
            Some(val) => Ok(val.clone()),
            None => Ok(OrderBookBidAsk {
                asks: vec![],
                bids: vec![],
            }),
        },
        None => Err(ConnectorError::DataError(result.error)),
    }
}

pub async fn get_median_price(pair: String, count: Option<i16>, window: f64) -> f64 {
    let order_book = get_order_book(pair, count).await.unwrap();

    let bids = order_book.bids;
    let asks = order_book.asks;
    let size = bids.len() + asks.len();
    let mut prices: Vec<f64> = Vec::with_capacity(size);
    let mut volumes: Vec<f64> = Vec::with_capacity(size);
    let mut timestamps: Vec<f64> = Vec::with_capacity(size);

    for (price, volume, timestamp) in asks {
        prices.push(price.to_string().parse::<f64>().unwrap());
        volumes.push(volume.to_string().parse::<f64>().unwrap());
        timestamps.push(timestamp);
    }

    for (price, volume, timestamp) in bids {
        prices.push(price.to_string().parse::<f64>().unwrap());
        volumes.push(-volume.to_string().parse::<f64>().unwrap());
        timestamps.push(timestamp);
    }

    let book_df = df!(
        "price" => prices,
        "volume" => volumes,
        "timestamp" => timestamps,
    )
    .unwrap();

    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
    let recent_book_df = book_df
        .lazy()
        .filter(col("timestamp").gt(lit(since_the_epoch - window)))
        .collect()
        .unwrap();

    recent_book_df["price"].median().unwrap()
}
