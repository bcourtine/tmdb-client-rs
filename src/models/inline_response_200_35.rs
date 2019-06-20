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
pub struct InlineResponse20035 {
    #[serde(rename = "episode_count", skip_serializing_if = "Option::is_none")]
    pub episode_count: Option<i32>,
    #[serde(rename = "group_count", skip_serializing_if = "Option::is_none")]
    pub group_count: Option<i32>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<::models::InlineResponse20035Groups>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<i32>,
    #[serde(rename = "network", skip_serializing_if = "Option::is_none")]
    pub network: Option<::models::InlineResponse20035Network>,
}

impl InlineResponse20035 {
    pub fn new() -> InlineResponse20035 {
        InlineResponse20035 {
            episode_count: None,
            group_count: None,
            name: None,
            description: None,
            groups: None,
            id: None,
            _type: None,
            network: None,
        }
    }
}


