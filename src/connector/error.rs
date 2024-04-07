use reqwest::Error as ReqwestError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum ConnectorError {
    #[error("request error")]
    RequestError(#[from] ReqwestError),
}
