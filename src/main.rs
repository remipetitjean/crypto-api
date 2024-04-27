mod connector;
use connector::kraken::account::account_balance::get_account_balance;
use connector::kraken::spot::add_order::add_order;
use connector::kraken::spot::market_data::order_book::get_median_price;
use rust_decimal::Decimal;
//use tokio::time::{sleep, Duration};

fn buy_gbpsol() {}

#[tokio::main]
async fn main() {
    //let server_time = get_server_time().await.unwrap();
    //println!("{}", server_time.unixtime);

    //let balance = get_account_balance().await.unwrap();
    //println!("{:?}", balance);

    // let mut i = 0;
    // while true {
    //     let account_balance = get_account_balance().await;
    //     println!("Account balance {} = {:?}\n\n", i, account_balance);
    //     sleep(Duration::from_millis(1001)).await;
    //     i += 1;
    // }

    // let extended_balance = get_extended_balance().await.unwrap();
    // println!("{:?}\n\n", extended_balance);

    // let trade_balance = get_trade_balance().await.unwrap();
    // println!("{:?}\n\n", trade_balance);

    // let closed_orders = get_closed_orders(true, None, None, None, None, None, None)
    //     .await
    //     .unwrap();
    // println!("{:?}\n\n", closed_orders);

    // let open_orders = get_open_orders(true, None).await.unwrap();
    // println!("{:?}\n\n", open_orders);

    // let query_orders = get_query_orders(
    //     "OTN6J7-NAYKU-TRIHU4;OO2PLV-OGQRJ-HMZNKB".to_string(),
    //     true,
    //     None,
    //     None,
    // )
    // .await
    // .unwrap();
    // println!("{:?}\n\n", query_orders);

    //let price = recent_book_df.get_column("price").median();

    let account_balance = get_account_balance().await.unwrap();
    let gbp = account_balance
        .get("ZGBP")
        .unwrap_or(&Decimal::ZERO)
        .to_string()
        .parse::<f64>()
        .unwrap();
    let sol = account_balance
        .get("SOL")
        .unwrap_or(&Decimal::ZERO)
        .to_string()
        .parse::<f64>()
        .unwrap();
    println!("gbp = {}, sol = {}", gbp, sol);

    let pair = "SOLGBP".to_string();
    let order_type = "limit".to_string();
    let buy_type = "buy".to_string();
    let sell_type = "sell".to_string();

    let price = get_median_price(pair.to_owned(), Some(500), 120.0)
        .await
        .to_string()
        .parse::<f64>()
        .unwrap();

    println!("price = {:?}", price);
    let fee = 0.0025;
    let greed = 0.005;
    let limit_price = price * (1. - fee - greed);
    let limit_price_dec = Decimal::new((limit_price * 100.) as i64, 2);
    let accepted_volume = gbp / limit_price;
    let accepted_volume_dec = Decimal::new((accepted_volume * 100_000_000.0) as i64, 8);
    println!("price = {} / {} {}", price, limit_price, limit_price_dec);
    println!("volume = {} / {}", gbp / price, accepted_volume_dec);

    let order = add_order(
        pair,
        order_type,
        buy_type,
        accepted_volume_dec,
        None,
        None,
        Some(limit_price_dec),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();
    println!("tx = {:?}", order);
}
