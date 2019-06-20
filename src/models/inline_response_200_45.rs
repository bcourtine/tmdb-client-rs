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
pub struct InlineResponse20045 {
    #[serde(rename = "person_results", skip_serializing_if = "Option::is_none")]
    pub person_results: Option<Vec<::models::PersonListResultsObject>>,
    #[serde(rename = "movie_results", skip_serializing_if = "Option::is_none")]
    pub movie_results: Option<Vec<::models::MovieListObject>>,
    #[serde(rename = "tv_results", skip_serializing_if = "Option::is_none")]
    pub tv_results: Option<Vec<::models::TvListResultObject>>,
    #[serde(rename = "tv_episode_results", skip_serializing_if = "Option::is_none")]
    pub tv_episode_results: Option<Vec<Value>>,
    #[serde(rename = "tv_season_results", skip_serializing_if = "Option::is_none")]
    pub tv_season_results: Option<Vec<Value>>,
}

impl InlineResponse20045 {
    pub fn new() -> InlineResponse20045 {
        InlineResponse20045 {
            person_results: None,
            movie_results: None,
            tv_results: None,
            tv_episode_results: None,
            tv_season_results: None,
        }
    }
}


