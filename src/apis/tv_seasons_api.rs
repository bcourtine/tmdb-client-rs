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

pub struct TVSeasonsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl TVSeasonsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> TVSeasonsApiClient {
        TVSeasonsApiClient {
            configuration: configuration,
        }
    }
}

pub trait TVSeasonsApi {
    fn get_tv_season_account_states(&self, tv_id: i32, season_number: i32, language: &str, guest_session_id: &str, session_id: &str) -> Result<crate::models::EpisodeRatingList, Error>;
    fn get_tv_season_changes(&self, season_id: i32, start_date: String, end_date: String, page: i32) -> Result<crate::models::ChangeDetails, Error>;
    fn get_tv_season_credits(&self, tv_id: i32, season_number: i32, language: &str) -> Result<crate::models::Credits, Error>;
    fn get_tv_season_details(&self, tv_id: i32, season_number: i32, language: &str, append_to_response: &str) -> Result<crate::models::SeasonDetails, Error>;
    fn get_tv_season_external_ids(&self, tv_id: i32, season_number: i32, language: &str) -> Result<crate::models::MovieTvExternalIds, Error>;
    fn get_tv_season_images(&self, tv_id: i32, season_number: i32, language: &str) -> Result<crate::models::Images, Error>;
    fn get_tv_season_videos(&self, tv_id: i32, season_number: i32, language: &str) -> Result<crate::models::VideosList, Error>;
}

impl TVSeasonsApi for TVSeasonsApiClient {
    fn get_tv_season_account_states(&self, tv_id: i32, season_number: i32, language: &str, guest_session_id: &str, session_id: &str) -> Result<crate::models::EpisodeRatingList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/tv/{tv_id}/season/{season_number}/account_states", configuration.base_path, tv_id=tv_id, season_number=season_number);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
        req_builder = req_builder.query(&[("guest_session_id", &guest_session_id.to_string())]);
        req_builder = req_builder.query(&[("session_id", &session_id.to_string())]);
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

    fn get_tv_season_changes(&self, season_id: i32, start_date: String, end_date: String, page: i32) -> Result<crate::models::ChangeDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/tv/season/{season_id}/changes", configuration.base_path, season_id=season_id);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("start_date", &start_date.to_string())]);
        req_builder = req_builder.query(&[("end_date", &end_date.to_string())]);
        req_builder = req_builder.query(&[("page", &page.to_string())]);
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

    fn get_tv_season_credits(&self, tv_id: i32, season_number: i32, language: &str) -> Result<crate::models::Credits, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/tv/{tv_id}/season/{season_number}/credits", configuration.base_path, tv_id=tv_id, season_number=season_number);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
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

    fn get_tv_season_details(&self, tv_id: i32, season_number: i32, language: &str, append_to_response: &str) -> Result<crate::models::SeasonDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/tv/{tv_id}/season/{season_number}", configuration.base_path, tv_id=tv_id, season_number=season_number);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
        req_builder = req_builder.query(&[("append_to_response", &append_to_response.to_string())]);
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

    fn get_tv_season_external_ids(&self, tv_id: i32, season_number: i32, language: &str) -> Result<crate::models::MovieTvExternalIds, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/tv/{tv_id}/season/{season_number}/external_ids", configuration.base_path, tv_id=tv_id, season_number=season_number);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
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

    fn get_tv_season_images(&self, tv_id: i32, season_number: i32, language: &str) -> Result<crate::models::Images, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/tv/{tv_id}/season/{season_number}/images", configuration.base_path, tv_id=tv_id, season_number=season_number);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
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

    fn get_tv_season_videos(&self, tv_id: i32, season_number: i32, language: &str) -> Result<crate::models::VideosList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/tv/{tv_id}/season/{season_number}/videos", configuration.base_path, tv_id=tv_id, season_number=season_number);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
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
