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
pub struct InlineResponse20058Logos {
    #[serde(rename = "aspect_ratio", skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f32>,
    #[serde(rename = "file_path", skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
    #[serde(rename = "file_type", skip_serializing_if = "Option::is_none")]
    pub file_type: Option<String>,
    #[serde(rename = "vote_average", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<i32>,
    #[serde(rename = "width", skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "vote_count", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i32>,
    #[serde(rename = "height", skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
}

impl InlineResponse20058Logos {
    pub fn new() -> InlineResponse20058Logos {
        InlineResponse20058Logos {
            aspect_ratio: None,
            file_path: None,
            file_type: None,
            vote_average: None,
            width: None,
            id: None,
            vote_count: None,
            height: None,
        }
    }
}

/// 
#[derive(Debug, Serialize, Deserialize)]
pub enum FileType {
    #[serde(rename = ".svg")]
    Svg,
    #[serde(rename = ".png")]
    Png,
}

