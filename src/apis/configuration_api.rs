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

use super::{Error, configuration};

pub struct ConfigurationApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ConfigurationApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ConfigurationApiClient {
        ConfigurationApiClient {
            configuration: configuration,
        }
    }
}

pub trait ConfigurationApi {
    fn get_configuration(&self, api_key: &str) -> Result<crate::models::Configuration, Error>;
}

impl ConfigurationApi for ConfigurationApiClient {
    fn get_configuration(&self, api_key: &str) -> Result<crate::models::Configuration, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/configuration", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("api_key", &api_key.to_string())]);
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