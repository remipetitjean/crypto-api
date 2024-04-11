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
        .expect("it can't fail")
        .as_millis()
}

pub fn get_api_sign(url: Url, secret: String, nonce: u128) -> String {
    // encode data
    let query = match url.query() {
        Some(query) => query,
        None => "",
    };
    let str_to_encode = format!("{nonce}{query}");
    let encoded = str_to_encode.as_bytes();
    let digest = sha2::Sha512::digest(encoded);
    let digest_str = str::from_utf8(&digest);
    println!("digest={:?}", digest_str);
    //encoded=b'1616492376594nonce=1616492376594&ordertype=limit&pair=XBTUSD&price=37500&type=buy&volume=1.25'

    let path = url.path();
    let mut message = path.as_bytes().to_vec();
    message.extend(digest);
    let message_str = str::from_utf8(&message);
    println!("message={:?}", message_str);

    let base64_secret = BASE64_STANDARD.decode(secret).unwrap();
    let key = hmac::Key::new(hmac::HMAC_SHA512, &base64_secret);
    let sig = hmac::sign(&key, &message);

    BASE64_STANDARD.encode(&sig)
    //message=b'/0/private/AddOrder#\xa1\xc1\xb3Lj\x11\xd6A\xaf\x0f$hH\x96\xcb\x90\xf6o\xb9\x91\x12\\\x83\xdc5{\xdc=\xc1F\xf1'
    //API-Sign: 4/dpxb3iT4tp/ZCVEwSnEsLxx0bqyhLpdfOpc6fn7OR8+UClSV5n9E6aSS8MPtnRfp32bAb0nmbRn6H8ndwLUQ==
}

#[cfg(test)]
mod tests {
    #[test]
    fn sample_api_sign() {
        let private_key = "kQH5HW/8p1uGOVjbgWA7FunAmGO8lsSUXNsu3eow76sz84Q18fWxnyRzBHCd3pd5nE9qa99HAZtuZuj6F1huXg==";
        let nonce: u128 = 1616492376594;
        let uri_path = "/0/private/AddOrder";
        //let url = Url::parse(&url_str).expect("could not parce URL");

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
