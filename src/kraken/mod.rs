use std::time::{SystemTime, UNIX_EPOCH};

/// kraken expects an increasing integer and suggests UNIX timestamp
/// (https://docs.kraken.com/rest/#section/Authentication/Nonce-and-2FA)
/// Should be ok as long as we don't execute multiple trades within ms
pub fn get_nonce() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("it can't fail")
        .as_millis()
}

pub mod health;
pub mod settings;
