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
pub struct InlineResponse20082 {
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(rename = "guest_session_id", skip_serializing_if = "Option::is_none")]
    pub guest_session_id: Option<String>,
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl InlineResponse20082 {
    pub fn new() -> InlineResponse20082 {
        InlineResponse20082 {
            expires_at: None,
            guest_session_id: None,
            success: None,
        }
    }
}


