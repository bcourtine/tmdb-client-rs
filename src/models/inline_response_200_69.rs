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
pub struct InlineResponse20069 {
    #[serde(rename = "include_adult", skip_serializing_if = "Option::is_none")]
    pub include_adult: Option<bool>,
    #[serde(rename = "iso_3166_1", skip_serializing_if = "Option::is_none")]
    pub iso_3166_1: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<::models::InlineResponse20069Avatar>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "iso_639_1", skip_serializing_if = "Option::is_none")]
    pub iso_639_1: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl InlineResponse20069 {
    pub fn new() -> InlineResponse20069 {
        InlineResponse20069 {
            include_adult: None,
            iso_3166_1: None,
            name: None,
            avatar: None,
            id: None,
            iso_639_1: None,
            username: None,
        }
    }
}


