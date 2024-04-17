mod connector;

use crate::connector::kraken::settings::KRAKEN_API_BASE_URL;
use connector::kraken::account::get_account_balance;
use connector::kraken::authentication::get_api_sign;
use connector::kraken::health::get_server_time;
use reqwest::Url;
use serde::Serialize;

#[derive(Serialize)]
struct AddOrderData {
    nonce: u128,
    ordertype: String,
    pair: String,
    price: u32,
    r#type: String,
    volume: f32,
}

#[tokio::main]
async fn main() {
    //let server_time = get_server_time().await.unwrap();
    //println!("{}", server_time.unixtime);

    //let balance = get_account_balance().await.unwrap();
    //println!("{:?}", balance);

    let account_balance = get_account_balance().await;
    println!("account balance={:?}", account_balance);

    //println!("{}", api_sign);
}
