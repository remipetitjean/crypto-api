use base64::prelude::*;
use reqwest::Url;
use ring::hmac;
use sha2::Digest;
use std::time::{SystemTime, UNIX_EPOCH};

pub const API_KEY_HEADER: &str = "API-Key";
pub const API_SIGN_HEADER: &str = "API-Sign";

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

    let base64_secret = BASE64_STANDARD.decode(secret).unwrap();
    let key = hmac::Key::new(hmac::HMAC_SHA512, &base64_secret);
    let sig = hmac::sign(&key, &message);
    BASE64_STANDARD.encode(&sig)
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
        let api_uri = "/0/private/AddOrder";
        let url_str = format!("{KRAKEN_API_BASE_URL}{api_uri}");
        let nonce: u128 = 1616492376594;
        let params = [
            ("nonce", nonce.to_string()),
            ("ordertype", "limit".to_string()),
            ("pair", "XBTUSD".to_string()),
            ("price", "37500".to_string()),
            ("type", "buy".to_string()),
            ("volume", "1.25".to_string()),
        ];
        let url = Url::parse_with_params(&url_str, &params).unwrap();
        let sig = get_api_sign(url, nonce, private_key);
        assert_eq!(
            sig,
            "4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==",
        );
    }
}
