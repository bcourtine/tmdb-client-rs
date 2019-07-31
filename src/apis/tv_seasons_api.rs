/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use std::option::Option;
use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{configuration, Error};

pub struct TVSeasonsApiClient<T: crate::apis::configuration::HttpClient + 'static> {
    configuration: Rc<configuration::Configuration<T>>,
}

impl <T: crate::apis::configuration::HttpClient + 'static> TVSeasonsApiClient<T> {
    pub fn new(configuration: Rc<configuration::Configuration<T>>) -> TVSeasonsApiClient<T> {
        TVSeasonsApiClient {
            configuration,
        }
    }
}

pub trait TVSeasonsApi {
    fn get_tv_season_account_states(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
    ) -> Result<crate::models::EpisodeRatingList, Error>;
    fn get_tv_season_changes(
        &self,
        season_id: i32,
        start_date: Option<String>,
        end_date: Option<String>,
        page: Option<i32>,
    ) -> Result<crate::models::ChangeDetails, Error>;
    fn get_tv_season_credits(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
    ) -> Result<crate::models::Credits, Error>;
    fn get_tv_season_details(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
        append_to_response: Option<&str>,
    ) -> Result<crate::models::SeasonDetails, Error>;
    fn get_tv_season_external_ids(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
    ) -> Result<crate::models::ExternalIds, Error>;
    fn get_tv_season_images(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
    ) -> Result<crate::models::Images, Error>;
    fn get_tv_season_videos(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
    ) -> Result<crate::models::VideosList, Error>;
}

impl <T: crate::apis::configuration::HttpClient + 'static> TVSeasonsApi for TVSeasonsApiClient<T> {
    fn get_tv_season_account_states(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
    ) -> Result<crate::models::EpisodeRatingList, Error> {
        let configuration: &configuration::Configuration<T> = self.configuration.borrow();
        let mut client = configuration.inner_client_guard();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/account_states",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = guest_session_id {
            req_builder = req_builder.query(&[("guest_session_id", &s.to_string())]);
        }
        if let Some(ref s) = session_id {
            req_builder = req_builder.query(&[("session_id", &s.to_string())]);
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

    fn get_tv_season_changes(
        &self,
        season_id: i32,
        start_date: Option<String>,
        end_date: Option<String>,
        page: Option<i32>,
    ) -> Result<crate::models::ChangeDetails, Error> {
        let configuration: &configuration::Configuration<T> = self.configuration.borrow();
        let mut client = configuration.inner_client_guard();

        let uri_str = format!(
            "{}/tv/season/{season_id}/changes",
            configuration.base_path,
            season_id = season_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = start_date {
            req_builder = req_builder.query(&[("start_date", &s.to_string())]);
        }
        if let Some(ref s) = end_date {
            req_builder = req_builder.query(&[("end_date", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
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

    fn get_tv_season_credits(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
    ) -> Result<crate::models::Credits, Error> {
        let configuration: &configuration::Configuration<T> = self.configuration.borrow();
        let mut client = configuration.inner_client_guard();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/credits",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number
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

    fn get_tv_season_details(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
        append_to_response: Option<&str>,
    ) -> Result<crate::models::SeasonDetails, Error> {
        let configuration: &configuration::Configuration<T> = self.configuration.borrow();
        let mut client = configuration.inner_client_guard();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = include_image_language {
            req_builder = req_builder.query(&[("include_image_language", &s.to_string())]);
        }
        if let Some(ref s) = append_to_response {
            req_builder = req_builder.query(&[("append_to_response", &s.to_string())]);
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

    fn get_tv_season_external_ids(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
    ) -> Result<crate::models::ExternalIds, Error> {
        let configuration: &configuration::Configuration<T> = self.configuration.borrow();
        let mut client = configuration.inner_client_guard();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/external_ids",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number
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

    fn get_tv_season_images(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
    ) -> Result<crate::models::Images, Error> {
        let configuration: &configuration::Configuration<T> = self.configuration.borrow();
        let mut client = configuration.inner_client_guard();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/images",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = include_image_language {
            req_builder = req_builder.query(&[("include_image_language", &s.to_string())]);
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

    fn get_tv_season_videos(
        &self,
        tv_id: i32,
        season_number: i32,
        language: Option<&str>,
    ) -> Result<crate::models::VideosList, Error> {
        let configuration: &configuration::Configuration<T> = self.configuration.borrow();
        let mut client = configuration.inner_client_guard();

        let uri_str = format!(
            "{}/tv/{tv_id}/season/{season_number}/videos",
            configuration.base_path,
            tv_id = tv_id,
            season_number = season_number
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
}
