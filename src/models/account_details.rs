/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AccountDetails {
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<crate::models::AccountdetailsAvatar>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "iso_639_1", skip_serializing_if = "Option::is_none")]
    pub iso_639_1: Option<String>,
    #[serde(rename = "iso_3166_1", skip_serializing_if = "Option::is_none")]
    pub iso_3166_1: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "include_adult", skip_serializing_if = "Option::is_none")]
    pub include_adult: Option<bool>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
