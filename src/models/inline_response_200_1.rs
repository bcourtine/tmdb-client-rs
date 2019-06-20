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
pub struct InlineResponse2001 {
    #[serde(rename = "iso_3166_1", skip_serializing_if = "Option::is_none")]
    pub iso_3166_1: Option<String>,
    #[serde(rename = "zones", skip_serializing_if = "Option::is_none")]
    pub zones: Option<Vec<String>>,
}

impl InlineResponse2001 {
    pub fn new() -> InlineResponse2001 {
        InlineResponse2001 {
            iso_3166_1: None,
            zones: None,
        }
    }
}


