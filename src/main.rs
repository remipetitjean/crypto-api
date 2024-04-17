mod connector;

use connector::kraken::account::account_balance::get_account_balance;

#[tokio::main]
async fn main() {
    //let server_time = get_server_time().await.unwrap();
    //println!("{}", server_time.unixtime);

    //let balance = get_account_balance().await.unwrap();
    //println!("{:?}", balance);

    let _account_balance = get_account_balance().await;

    //println!("{}", api_sign);
}
