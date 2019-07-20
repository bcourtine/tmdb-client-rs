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
pub struct Images {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "backdrops", skip_serializing_if = "Option::is_none")]
    pub backdrops: Option<Vec<crate::models::Image>>,
    #[serde(rename = "posters", skip_serializing_if = "Option::is_none")]
    pub posters: Option<Vec<crate::models::Image>>,
    #[serde(rename = "logos", skip_serializing_if = "Option::is_none")]
    pub logos: Option<Vec<crate::models::Image>>,
}

impl Images {
    pub fn new() -> Images {
        Images {
            id: None,
            backdrops: None,
            posters: None,
            logos: None,
        }
    }
}
