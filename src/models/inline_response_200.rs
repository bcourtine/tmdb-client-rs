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
pub struct InlineResponse200 {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(rename = "total_pages", skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i32>,
    #[serde(rename = "results", skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<::models::TvListResultObject>>,
    #[serde(rename = "total_results", skip_serializing_if = "Option::is_none")]
    pub total_results: Option<i32>,
}

impl InlineResponse200 {
    pub fn new() -> InlineResponse200 {
        InlineResponse200 {
            page: None,
            total_pages: None,
            results: None,
            total_results: None,
        }
    }
}


