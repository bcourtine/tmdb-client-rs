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

pub struct PeopleApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl PeopleApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> PeopleApiClient {
        PeopleApiClient {
            configuration: configuration,
        }
    }
}

pub trait PeopleApi {
    fn get_person_changes(&self, person_id: i32, language: &str, start_date: String, end_date: String, page: i32) -> Result<crate::models::ChangeDetails, Error>;
    fn get_person_combined_credits(&self, person_id: i32, language: &str) -> Result<crate::models::PersonCredits, Error>;
    fn get_person_details(&self, person_id: i32, language: &str, append_to_response: &str) -> Result<crate::models::PersonDetails, Error>;
    fn get_person_external_ids(&self, person_id: i32, language: &str) -> Result<crate::models::PersonExternalIds, Error>;
    fn get_person_images_list(&self, person_id: i32) -> Result<crate::models::PersonImagesList, Error>;
    fn get_person_latest_details(&self, language: &str) -> Result<crate::models::PersonDetails, Error>;
    fn get_person_movie_credits(&self, person_id: i32, language: &str) -> Result<crate::models::PersonCredits, Error>;
    fn get_person_popular_paginated(&self, language: &str, page: i32) -> Result<crate::models::PersonPopularPaginated, Error>;
    fn get_person_tagged_images_paginated(&self, person_id: i32, language: &str, page: i32) -> Result<crate::models::PersonTaggedImagesPaginated, Error>;
    fn get_person_tv_credits(&self, person_id: i32, language: &str) -> Result<crate::models::PersonCredits, Error>;
}

impl PeopleApi for PeopleApiClient {
    fn get_person_changes(&self, person_id: i32, language: &str, start_date: String, end_date: String, page: i32) -> Result<crate::models::ChangeDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/{person_id}/changes", configuration.base_path, person_id=person_id);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
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

    fn get_person_combined_credits(&self, person_id: i32, language: &str) -> Result<crate::models::PersonCredits, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/{person_id}/combined_credits", configuration.base_path, person_id=person_id);
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

    fn get_person_details(&self, person_id: i32, language: &str, append_to_response: &str) -> Result<crate::models::PersonDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/{person_id}", configuration.base_path, person_id=person_id);
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

    fn get_person_external_ids(&self, person_id: i32, language: &str) -> Result<crate::models::PersonExternalIds, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/{person_id}/external_ids", configuration.base_path, person_id=person_id);
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

    fn get_person_images_list(&self, person_id: i32) -> Result<crate::models::PersonImagesList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/{person_id}/images", configuration.base_path, person_id=person_id);
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

    fn get_person_latest_details(&self, language: &str) -> Result<crate::models::PersonDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/latest", configuration.base_path);
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

    fn get_person_movie_credits(&self, person_id: i32, language: &str) -> Result<crate::models::PersonCredits, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/{person_id}/movie_credits", configuration.base_path, person_id=person_id);
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

    fn get_person_popular_paginated(&self, language: &str, page: i32) -> Result<crate::models::PersonPopularPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/popular", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
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

    fn get_person_tagged_images_paginated(&self, person_id: i32, language: &str, page: i32) -> Result<crate::models::PersonTaggedImagesPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/{person_id}/tagged_images", configuration.base_path, person_id=person_id);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
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

    fn get_person_tv_credits(&self, person_id: i32, language: &str) -> Result<crate::models::PersonCredits, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/{person_id}/tv_credits", configuration.base_path, person_id=person_id);
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