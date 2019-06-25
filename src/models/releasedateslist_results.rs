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
pub struct ReleasedateslistResults {
    #[serde(rename = "release_dates", skip_serializing_if = "Option::is_none")]
    pub release_dates: Option<Vec<crate::models::ReleaseDate>>,
    #[serde(rename = "iso_3166_1", skip_serializing_if = "Option::is_none")]
    pub iso_3166_1: Option<String>,
}

impl ReleasedateslistResults {
    pub fn new() -> ReleasedateslistResults {
        ReleasedateslistResults {
            release_dates: None,
            iso_3166_1: None,
        }
    }
}


