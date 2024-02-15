use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct TezaursError {
    pub error: Error,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Error {
    #[serde(alias = "error_code")]
    pub code: i32,
    #[serde(alias = "error_msg")]
    pub message: String,
    pub request_params: Vec<RequestParam>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RequestParam {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub enum TezaursApiError {
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    //VkError(VkError),
    //InternalError(String),
}

impl From<reqwest::Error> for TezaursApiError {
    fn from(error: reqwest::Error) -> Self {
        TezaursApiError::ReqwestError(error)
    }
}

impl From<serde_json::Error> for TezaursApiError {
    fn from(error: serde_json::Error) -> Self {
        TezaursApiError::SerdeJsonError(error)
    }
}

impl fmt::Display for TezaursApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TezaursApiError::ReqwestError(error) => write!(f, "Reqwest Error: {}", error),
            TezaursApiError::SerdeJsonError(error) => write!(f, "Serde JSON Error: {}", error),
            //VkApiError::VkError(response) => write!(f, "VK Error: {:?}", response),
            //VkApiError::InternalError(response) => write!(f, "Internal Error: {:?}", response),
        }
    }
}

impl std::error::Error for TezaursApiError {}