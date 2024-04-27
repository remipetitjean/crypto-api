use reqwest::Error as ReqwestError;
use thiserror::Error as ThisError;

// https://support.kraken.com/hc/en-us/articles/360001491786-API-error-messages

#[derive(ThisError, Debug)]
pub enum ConnectorError {
    #[error("request error")]
    RequestError(#[from] ReqwestError),

    #[error("data error")]
    DataError(Vec<String>),
}
