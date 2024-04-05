mod kraken;

use kraken::health::get_server_time;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    get_server_time().await;
}
