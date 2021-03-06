/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
use std::rc::Rc;

use reqwest;

use super::configuration;
use crate::Error;

pub struct TrendingApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl TrendingApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> TrendingApiClient {
        TrendingApiClient {
            configuration,
        }
    }
}

pub trait TrendingApi {
    fn get_trending_paginated(
        &self,
        media_type: &crate::models::MediaType,
        time_window: &crate::models::TimeWindow,
    ) -> Result<crate::models::SearchMultiResultsPaginated, Error>;
}

impl TrendingApi for TrendingApiClient {
    fn get_trending_paginated(
        &self,
        media_type: &crate::models::MediaType,
        time_window: &crate::models::TimeWindow,
    ) -> Result<crate::models::SearchMultiResultsPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/trending/{media_type}/{time_window}",
            configuration.base_path,
            media_type = media_type.to_string().to_lowercase(),
            time_window = time_window.to_string().to_lowercase(),
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref apikey) = configuration.api_key {
            req_builder = req_builder.query(&[("api_key", apikey)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
