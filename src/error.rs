#![allow(dead_code)]
#![allow(unused_variables)]

//use serde_json;
use core::fmt;
use std::error::Error;

pub type APIResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub enum HuobiError {
    ApiError(String),
}

impl fmt::Display for HuobiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.clone() {
            HuobiError::ApiError(why) => write!(f, "ApiError: {}", why),
        }
    }
}

impl Error for HuobiError {
    fn description(&self) -> &str {
        "HBDM Swap Error"
    }
}
