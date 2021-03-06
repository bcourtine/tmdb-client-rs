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

pub struct CertificationsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl CertificationsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> CertificationsApiClient {
        CertificationsApiClient {
            configuration,
        }
    }
}

pub trait CertificationsApi {
    fn get_movie_certifications_list(&self) -> Result<crate::models::Certifications, Error>;
    fn get_tv_certifications_list(&self) -> Result<crate::models::Certifications, Error>;
}

impl CertificationsApi for CertificationsApiClient {
    fn get_movie_certifications_list(&self) -> Result<crate::models::Certifications, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/certification/movie/list", configuration.base_path);
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

    fn get_tv_certifications_list(&self) -> Result<crate::models::Certifications, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/certification/tv/list", configuration.base_path);
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
