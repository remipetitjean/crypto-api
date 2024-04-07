mod connector;

use connector::kraken::health::get_server_time;

#[tokio::main]
async fn main() {
    let server_time = get_server_time().await.unwrap();
    println!("{}", server_time.unixtime);
}
