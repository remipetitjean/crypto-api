mod connector;

use connector::kraken::account::get_account_balance;

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
