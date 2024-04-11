mod connector;

use crate::connector::kraken::settings::KRAKEN_API_BASE_URL;
use connector::kraken::account::get_account_balance;
use connector::kraken::authentication::get_api_sign;
use connector::kraken::health::get_server_time;
use reqwest::Url;

#[tokio::main]
async fn main() {
    //let server_time = get_server_time().await.unwrap();
    //println!("{}", server_time.unixtime);

    //let balance = get_account_balance().await.unwrap();
    //println!("{:?}", balance);

    let private_key =
        "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg=="
            .to_string();
    let nonce: u128 = 1616492376594;
    let api_uri = "/0/private/AddOrder";
    let url_str = format!("{KRAKEN_API_BASE_URL}{api_uri}");
    let params = [
        ("nonce", nonce.to_string()),
        ("ordertype", "limit".to_string()),
        ("pair", "XBTUSD".to_string()),
        ("price", "37500".to_string()),
        ("type", "buy".to_string()),
        ("volume", "1.25".to_string()),
    ];
    let url = Url::parse_with_params(&url_str, &params).expect("could not parce URL");

    let api_sign = get_api_sign(url, private_key, nonce);
    println!("{}", api_sign);
}
