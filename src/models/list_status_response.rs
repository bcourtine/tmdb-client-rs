/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListStatusResponse {
    #[serde(rename = "status_message", skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
    #[serde(rename = "status_code", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(rename = "list_id", skip_serializing_if = "Option::is_none")]
    pub list_id: Option<i32>,
}

impl ListStatusResponse {
    pub fn new() -> ListStatusResponse {
        ListStatusResponse {
            status_message: None,
            success: None,
            status_code: None,
            list_id: None,
        }
    }
}
