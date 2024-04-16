use super::settings::KRAKEN_API_SECRET;
use base64::prelude::*;
use reqwest::Url;
use ring::hmac;
use serde::Deserialize;
use sha2::{Digest, Sha512};
use std::collections::HashMap;
use std::str;
use std::time::{SystemTime, UNIX_EPOCH};

/// kraken expects an increasing integer and suggests UNIX timestamp
/// (https://docs.kraken.com/rest/#section/Authentication/Nonce-and-2FA)
/// Should be ok as long as we don't execute multiple trades within ms
pub fn get_nonce() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

pub fn get_api_sign(url: Url, nonce: u128, secret: String) -> String {
    let query = match url.query() {
        Some(query) => query,
        None => "",
    };
    let str_to_encode = format!("{nonce}{query}");
    let encoded = str_to_encode.into_bytes();

    let path = url.path();
    let mut message = path.as_bytes().to_vec();
    message.extend(sha2::Sha256::digest(encoded));

    let key = hmac::Key::new(hmac::HMAC_SHA512, &base64::decode(secret).unwrap());
    let sig = hmac::sign(&key, &message);
    base64::encode(&sig)
}

#[cfg(test)]
mod tests {
    use super::super::settings::KRAKEN_API_BASE_URL;
    use super::*;

    #[test]
    fn sample_api_sign() {
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
        let sig_2 = get_api_sign(url, nonce, private_key);
        assert_eq!(
            sig_2,
            "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==",
        );
    }
}
