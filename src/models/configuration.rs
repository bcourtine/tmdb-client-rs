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
pub struct Configuration {
    #[serde(rename = "images", skip_serializing_if = "Option::is_none")]
    pub images: Option<crate::models::ConfigurationImages>,
    #[serde(rename = "change_keys", skip_serializing_if = "Option::is_none")]
    pub change_keys: Option<Vec<String>>,
}

impl Configuration {
    pub fn new() -> Configuration {
        Configuration {
            images: None,
            change_keys: None,
        }
    }
}
