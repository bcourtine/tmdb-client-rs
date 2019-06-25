/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{Error, configuration, urlencode};

pub struct CreditsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl CreditsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> CreditsApiClient {
        CreditsApiClient {
            configuration: configuration,
        }
    }
}

pub trait CreditsApi {
    fn get_credit_details(&self, credit_id: &str) -> Result<crate::models::Credit, Error>;
}

impl CreditsApi for CreditsApiClient {
    fn get_credit_details(&self, credit_id: &str) -> Result<crate::models::Credit, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/credit/{credit_id}", configuration.base_path, credit_id=urlencode(credit_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("api_key", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
