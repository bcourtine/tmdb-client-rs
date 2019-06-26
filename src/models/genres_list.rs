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
pub struct GenresList {
    #[serde(rename = "genres", skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<crate::models::Genre>>,
}

impl GenresList {
    pub fn new() -> GenresList {
        GenresList { genres: None }
    }
}
