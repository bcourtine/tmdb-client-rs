/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;


#[derive(Debug, Serialize, Deserialize)]
pub struct CreditMedia {
    #[serde(rename = "character", skip_serializing_if = "Option::is_none")]
    pub character: Option<String>,
    #[serde(rename = "seasons", skip_serializing_if = "Option::is_none")]
    pub seasons: Option<Vec<crate::models::CreditMediaSeasons>>,
    #[serde(rename = "original_name", skip_serializing_if = "Option::is_none")]
    pub original_name: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "episodes", skip_serializing_if = "Option::is_none")]
    pub episodes: Option<Vec<Value>>,
}

impl CreditMedia {
    pub fn new() -> CreditMedia {
        CreditMedia {
            character: None,
            seasons: None,
            original_name: None,
            name: None,
            id: None,
            episodes: None,
        }
    }
}


