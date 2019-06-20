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
pub struct InlineObject7 {
    #[serde(rename = "media_id")]
    pub media_id: i32,
}

impl InlineObject7 {
    pub fn new(media_id: i32) -> InlineObject7 {
        InlineObject7 {
            media_id: media_id,
        }
    }
}


