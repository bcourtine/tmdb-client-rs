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

pub struct AuthenticationApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl AuthenticationApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> AuthenticationApiClient {
        AuthenticationApiClient {
            configuration: configuration,
        }
    }
}

pub trait AuthenticationApi {
    fn get_new_authentication_guest_session(&self, ) -> Result<crate::models::GuestSessionResponse, Error>;
    fn get_new_authentication_session(&self, request_token: &str) -> Result<crate::models::SessionResponse, Error>;
    fn get_new_authentication_token(&self, ) -> Result<crate::models::TokenResponseWithExpiration, Error>;
    fn get_validate_authentication_token_with_login(&self, username: &str, password: &str, request_token: &str) -> Result<crate::models::TokenResponse, Error>;
}

impl AuthenticationApi for AuthenticationApiClient {
    fn get_new_authentication_guest_session(&self, ) -> Result<crate::models::GuestSessionResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/authentication/guest_session/new", configuration.base_path);
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

    fn get_new_authentication_session(&self, request_token: &str) -> Result<crate::models::SessionResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/authentication/session/new", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("request_token", &request_token.to_string())]);
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

    fn get_new_authentication_token(&self, ) -> Result<crate::models::TokenResponseWithExpiration, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/authentication/token/new", configuration.base_path);
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

    fn get_validate_authentication_token_with_login(&self, username: &str, password: &str, request_token: &str) -> Result<crate::models::TokenResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/authentication/token/validate_with_login", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("username", &username.to_string())]);
        req_builder = req_builder.query(&[("password", &password.to_string())]);
        req_builder = req_builder.query(&[("request_token", &request_token.to_string())]);
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