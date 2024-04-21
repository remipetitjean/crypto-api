mod connector;

use connector::kraken::account::account_balance::get_account_balance;
use connector::kraken::account::closed_orders::get_closed_orders;
use connector::kraken::account::extended_balance::get_extended_balance;
use connector::kraken::account::open_orders::get_open_orders;
use connector::kraken::account::query_orders::get_query_orders;
use connector::kraken::account::trade_balance::get_trade_balance;

#[tokio::main]
async fn main() {
    //let server_time = get_server_time().await.unwrap();
    //println!("{}", server_time.unixtime);

    //let balance = get_account_balance().await.unwrap();
    //println!("{:?}", balance);

    let account_balance = get_account_balance().await.unwrap();
    println!("{:?}\n\n", account_balance);

    let extended_balance = get_extended_balance().await.unwrap();
    println!("{:?}\n\n", extended_balance);

    let trade_balance = get_trade_balance().await.unwrap();
    println!("{:?}\n\n", trade_balance);

    let open_orders = get_open_orders(true, None).await.unwrap();
    println!("{:?}\n\n", open_orders);

    let closed_orders = get_closed_orders(true, None, None, None, None, None, None)
        .await
        .unwrap();
    println!("{:?}\n\n", closed_orders);

    let query_orders = get_query_orders("OTN6J7-NAYKU-TRIHU4".to_string(), true, None, None)
        .await
        .unwrap();
    println!("{:?}\n\n", query_orders);
}
