#[macro_use]
extern crate serde_derive;

#[cfg(feature = "blocking")]
pub mod apis;

pub mod models;

#[cfg(feature = "daily-exports")]
pub mod files;

#[cfg(feature = "async")]
pub mod async_apis;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP request error")]
    Reqwest(#[from] reqwest::Error),
    
    #[error("JSON serialization/deserialization error")]
    Serde(#[from] serde_json::Error),
    
    #[error("I/O error")]
    Io(#[from] std::io::Error),
}
