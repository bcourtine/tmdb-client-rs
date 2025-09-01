#[macro_use]
extern crate serde_derive;

pub mod apis;
pub mod models;
pub mod files;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("HTTP request error")]
    Reqwest(#[from] reqwest::Error),
    
    #[error("JSON serialization/deserialization error")]
    Serde(#[from] serde_json::Error),
    
    #[error("I/O error")]
    Io(#[from] std::io::Error),
}
