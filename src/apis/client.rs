use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    configuration: Rc<Configuration>,
    account_api: Box<crate::apis::AccountApi>,
    authentication_api: Box<crate::apis::AuthenticationApi>,
    certifications_api: Box<crate::apis::CertificationsApi>,
    changes_api: Box<crate::apis::ChangesApi>,
    collections_api: Box<crate::apis::CollectionsApi>,
    companies_api: Box<crate::apis::CompaniesApi>,
    configuration_api: Box<crate::apis::ConfigurationApi>,
    credits_api: Box<crate::apis::CreditsApi>,
    discover_api: Box<crate::apis::DiscoverApi>,
    find_api: Box<crate::apis::FindApi>,
    genres_api: Box<crate::apis::GenresApi>,
    guest_sessions_api: Box<crate::apis::GuestSessionsApi>,
    jobs_api: Box<crate::apis::JobsApi>,
    keywords_api: Box<crate::apis::KeywordsApi>,
    lists_api: Box<crate::apis::ListsApi>,
    movies_api: Box<crate::apis::MoviesApi>,
    networks_api: Box<crate::apis::NetworksApi>,
    people_api: Box<crate::apis::PeopleApi>,
    reviews_api: Box<crate::apis::ReviewsApi>,
    search_api: Box<crate::apis::SearchApi>,
    tv_api: Box<crate::apis::TVApi>,
    tv_episodes_api: Box<crate::apis::TVEpisodesApi>,
    tv_seasons_api: Box<crate::apis::TVSeasonsApi>,
    timezones_api: Box<crate::apis::TimezonesApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            account_api: Box::new(crate::apis::AccountApiClient::new(rc.clone())),
            authentication_api: Box::new(crate::apis::AuthenticationApiClient::new(rc.clone())),
            certifications_api: Box::new(crate::apis::CertificationsApiClient::new(rc.clone())),
            changes_api: Box::new(crate::apis::ChangesApiClient::new(rc.clone())),
            collections_api: Box::new(crate::apis::CollectionsApiClient::new(rc.clone())),
            companies_api: Box::new(crate::apis::CompaniesApiClient::new(rc.clone())),
            configuration_api: Box::new(crate::apis::ConfigurationApiClient::new(rc.clone())),
            credits_api: Box::new(crate::apis::CreditsApiClient::new(rc.clone())),
            discover_api: Box::new(crate::apis::DiscoverApiClient::new(rc.clone())),
            find_api: Box::new(crate::apis::FindApiClient::new(rc.clone())),
            genres_api: Box::new(crate::apis::GenresApiClient::new(rc.clone())),
            guest_sessions_api: Box::new(crate::apis::GuestSessionsApiClient::new(rc.clone())),
            jobs_api: Box::new(crate::apis::JobsApiClient::new(rc.clone())),
            keywords_api: Box::new(crate::apis::KeywordsApiClient::new(rc.clone())),
            lists_api: Box::new(crate::apis::ListsApiClient::new(rc.clone())),
            movies_api: Box::new(crate::apis::MoviesApiClient::new(rc.clone())),
            networks_api: Box::new(crate::apis::NetworksApiClient::new(rc.clone())),
            people_api: Box::new(crate::apis::PeopleApiClient::new(rc.clone())),
            reviews_api: Box::new(crate::apis::ReviewsApiClient::new(rc.clone())),
            search_api: Box::new(crate::apis::SearchApiClient::new(rc.clone())),
            tv_api: Box::new(crate::apis::TVApiClient::new(rc.clone())),
            tv_episodes_api: Box::new(crate::apis::TVEpisodesApiClient::new(rc.clone())),
            tv_seasons_api: Box::new(crate::apis::TVSeasonsApiClient::new(rc.clone())),
            timezones_api: Box::new(crate::apis::TimezonesApiClient::new(rc.clone())),
        }
    }

    pub fn new_with_api_key<T: Into<String>>(api_key: T) -> APIClient {
        let configuration = Configuration::new_with_api_key(api_key);
        APIClient::new(configuration)
    }

    pub fn new_from_env() -> APIClient {
        let api_key = env!("TMDB_API_KEY");
        APIClient::new_with_api_key(api_key)
    }

    pub fn account_api(&self) -> &crate::apis::AccountApi {
        self.account_api.as_ref()
    }

    pub fn authentication_api(&self) -> &crate::apis::AuthenticationApi {
        self.authentication_api.as_ref()
    }

    pub fn certifications_api(&self) -> &crate::apis::CertificationsApi {
        self.certifications_api.as_ref()
    }

    pub fn changes_api(&self) -> &crate::apis::ChangesApi {
        self.changes_api.as_ref()
    }

    pub fn collections_api(&self) -> &crate::apis::CollectionsApi {
        self.collections_api.as_ref()
    }

    pub fn companies_api(&self) -> &crate::apis::CompaniesApi {
        self.companies_api.as_ref()
    }

    pub fn configuration_api(&self) -> &crate::apis::ConfigurationApi {
        self.configuration_api.as_ref()
    }

    pub fn credits_api(&self) -> &crate::apis::CreditsApi {
        self.credits_api.as_ref()
    }

    pub fn discover_api(&self) -> &crate::apis::DiscoverApi {
        self.discover_api.as_ref()
    }

    pub fn find_api(&self) -> &crate::apis::FindApi {
        self.find_api.as_ref()
    }

    pub fn genres_api(&self) -> &crate::apis::GenresApi {
        self.genres_api.as_ref()
    }

    pub fn guest_sessions_api(&self) -> &crate::apis::GuestSessionsApi {
        self.guest_sessions_api.as_ref()
    }

    pub fn jobs_api(&self) -> &crate::apis::JobsApi {
        self.jobs_api.as_ref()
    }

    pub fn keywords_api(&self) -> &crate::apis::KeywordsApi {
        self.keywords_api.as_ref()
    }

    pub fn lists_api(&self) -> &crate::apis::ListsApi {
        self.lists_api.as_ref()
    }

    pub fn movies_api(&self) -> &crate::apis::MoviesApi {
        self.movies_api.as_ref()
    }

    pub fn networks_api(&self) -> &crate::apis::NetworksApi {
        self.networks_api.as_ref()
    }

    pub fn people_api(&self) -> &crate::apis::PeopleApi {
        self.people_api.as_ref()
    }

    pub fn reviews_api(&self) -> &crate::apis::ReviewsApi {
        self.reviews_api.as_ref()
    }

    pub fn search_api(&self) -> &crate::apis::SearchApi {
        self.search_api.as_ref()
    }

    pub fn tv_api(&self) -> &crate::apis::TVApi {
        self.tv_api.as_ref()
    }

    pub fn tv_episodes_api(&self) -> &crate::apis::TVEpisodesApi {
        self.tv_episodes_api.as_ref()
    }

    pub fn tv_seasons_api(&self) -> &crate::apis::TVSeasonsApi {
        self.tv_seasons_api.as_ref()
    }

    pub fn timezones_api(&self) -> &crate::apis::TimezonesApi {
        self.timezones_api.as_ref()
    }
}
