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
pub struct InlineResponse20064 {
    #[serde(rename = "headquarters", skip_serializing_if = "Option::is_none")]
    pub headquarters: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "origin_country", skip_serializing_if = "Option::is_none")]
    pub origin_country: Option<String>,
}

impl InlineResponse20064 {
    pub fn new() -> InlineResponse20064 {
        InlineResponse20064 {
            headquarters: None,
            name: None,
            id: None,
            homepage: None,
            origin_country: None,
        }
    }
}


