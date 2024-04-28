use crate::connector::error::ConnectorError;
use crate::connector::kraken::request::send_public_request;
use crate::connector::kraken::settings::KRAKEN_API_BASE_URL;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;

use polars::prelude::*;
use rust_decimal::Decimal;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
struct Request {
    pair: String,
    count: Option<i16>,
}

type Order = (Decimal, Decimal, f64);

#[derive(Debug, Deserialize, Clone)]
struct BidAskOrderBook {
    asks: Vec<Order>,
    bids: Vec<Order>,
}

type BidAskOrderBookMap = HashMap<String, BidAskOrderBook>;

pub async fn get_order_book(pair: String) -> Result<BidAskOrderBook, ConnectorError> {
    let request_data = Request {
        pair: pair.clone(),
        count: Some(500),
    };
    let endpoint = "Depth".to_string();
    let data = send_public_request::<Request, BidAskOrderBookMap>(endpoint, request_data).await?;
    match data.get(&pair) {
        Some(res) => Ok(res.clone()),
        None => Ok(BidAskOrderBook {
            asks: vec![],
            bids: vec![],
        }),
    }
}

fn book_vec_to_df(book_vec: Vec<Order>, window: Option<f64>) -> DataFrame {
    let size = book_vec.len();
    let mut prices: Vec<f64> = Vec::with_capacity(size);
    let mut volumes: Vec<f64> = Vec::with_capacity(size);
    let mut timestamps: Vec<f64> = Vec::with_capacity(size);

    for (price, volume, timestamp) in book_vec {
        prices.push(price.to_string().parse::<f64>().unwrap());
        volumes.push(volume.to_string().parse::<f64>().unwrap());
        timestamps.push(timestamp);
    }

    let df = df! {
        "price" => prices,
        "volume" => volumes,
        "timestamp" => timestamps,
    }
    .unwrap();
    match window {
        Some(window) => {
            let now = SystemTime::now();
            let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
            df.lazy()
                .filter(col("timestamp").gt(lit(since_the_epoch - window)))
                .collect()
                .unwrap()
        }
        None => df,
    }
}

pub async fn get_book_df(pair: String, window: Option<f64>) -> DataFrame {
    let order_book = get_order_book(pair).await.unwrap();
    let bid_df = book_vec_to_df(order_book.bids, window)
        .lazy()
        .select([
            lit("bid").alias("bid_ask"),
            col("price"),
            col("volume"),
            col("timestamp"),
        ])
        .collect()
        .unwrap();
    let ask_df = book_vec_to_df(order_book.asks, window)
        .lazy()
        .select([
            lit("ask").alias("bid_ask"),
            col("price"),
            col("volume"),
            col("timestamp"),
        ])
        .collect()
        .unwrap();
    concat([bid_df.lazy(), ask_df.lazy()], UnionArgs::default())
        .unwrap()
        .collect()
        .unwrap()
}

pub async fn get_bid_book_df(pair: String, window: Option<f64>) -> DataFrame {
    let order_book = get_order_book(pair).await.unwrap();
    book_vec_to_df(order_book.bids, window)
        .lazy()
        .select([col("price"), col("volume"), col("timestamp")])
        .collect()
        .unwrap()
}

pub async fn get_ask_book_df(pair: String, window: Option<f64>) -> DataFrame {
    let order_book = get_order_book(pair).await.unwrap();
    book_vec_to_df(order_book.asks, window)
        .lazy()
        .select([col("price"), col("volume"), col("timestamp")])
        .collect()
        .unwrap()
}

pub async fn get_median_price(pair: String, window: Option<f64>) -> f64 {
    let book_df = get_book_df(pair, window).await;
    book_df["price"].median().unwrap()
}

pub async fn get_bid_max_price(pair: String, window: Option<f64>) -> f64 {
    let book_df = get_book_df(pair, window).await;
    let bid_df = book_df
        .lazy()
        .filter(col("bid_ask").eq(lit("bid")))
        .collect()
        .unwrap();
    println!("bid_max_price = {:?}", bid_df);
    1.1
}
