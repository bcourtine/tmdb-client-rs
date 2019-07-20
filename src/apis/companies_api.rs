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
use std::option::Option;
use std::rc::Rc;

use reqwest;

use super::{configuration, Error};

pub struct CompaniesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl CompaniesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> CompaniesApiClient {
        CompaniesApiClient {
            configuration: configuration,
        }
    }
}

pub trait CompaniesApi {
    fn get_company_details(&self, company_id: i32) -> Result<crate::models::CompanyDetails, Error>;
    fn get_company_movies_paginated(
        &self,
        company_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_company_images(&self, company_id: i32) -> Result<crate::models::Images, Error>;
}

impl CompaniesApi for CompaniesApiClient {
    fn get_company_details(&self, company_id: i32) -> Result<crate::models::CompanyDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/company/{company_id}",
            configuration.base_path,
            company_id = company_id
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

    fn get_company_movies_paginated(
        &self,
        company_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/company/{company_id}/movies",
            configuration.base_path,
            company_id = company_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
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

    fn get_company_images(&self, company_id: i32) -> Result<crate::models::Images, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/company/{company_id}/images",
            configuration.base_path,
            company_id = company_id
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
