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
pub struct MediaIdBody {
    #[serde(rename = "media_id")]
    pub media_id: i32,
}

impl MediaIdBody {
    pub fn new(media_id: i32) -> MediaIdBody {
        MediaIdBody { media_id: media_id }
    }
}