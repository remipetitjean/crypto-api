mod connector;

use connector::kraken::spot::market_data::order_book::get_median_price;
//use tokio::time::{sleep, Duration};

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
    let median_price = get_median_price("SOLGBP".to_string(), Some(500), 120.0).await;
    println!("median price = {:?}", median_price);
}
