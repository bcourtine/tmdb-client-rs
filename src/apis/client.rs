use std::env;
use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    configuration: Rc<Configuration>,
    account_api: Box<dyn crate::apis::AccountApi>,
    authentication_api: Box<dyn crate::apis::AuthenticationApi>,
    certifications_api: Box<dyn crate::apis::CertificationsApi>,
    changes_api: Box<dyn crate::apis::ChangesApi>,
    collections_api: Box<dyn crate::apis::CollectionsApi>,
    companies_api: Box<dyn crate::apis::CompaniesApi>,
    configuration_api: Box<dyn crate::apis::ConfigurationApi>,
    credits_api: Box<dyn crate::apis::CreditsApi>,
    discover_api: Box<dyn crate::apis::DiscoverApi>,
    find_api: Box<dyn crate::apis::FindApi>,
    genres_api: Box<dyn crate::apis::GenresApi>,
    guest_sessions_api: Box<dyn crate::apis::GuestSessionsApi>,
    keywords_api: Box<dyn crate::apis::KeywordsApi>,
    lists_api: Box<dyn crate::apis::ListsApi>,
    movies_api: Box<dyn crate::apis::MoviesApi>,
    networks_api: Box<dyn crate::apis::NetworksApi>,
    people_api: Box<dyn crate::apis::PeopleApi>,
    reviews_api: Box<dyn crate::apis::ReviewsApi>,
    search_api: Box<dyn crate::apis::SearchApi>,
    trending_api: Box<dyn crate::apis::TrendingApi>,
    tv_api: Box<dyn crate::apis::TVApi>,
    tv_episodes_api: Box<dyn crate::apis::TVEpisodesApi>,
    tv_seasons_api: Box<dyn crate::apis::TVSeasonsApi>,
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
            keywords_api: Box::new(crate::apis::KeywordsApiClient::new(rc.clone())),
            lists_api: Box::new(crate::apis::ListsApiClient::new(rc.clone())),
            movies_api: Box::new(crate::apis::MoviesApiClient::new(rc.clone())),
            networks_api: Box::new(crate::apis::NetworksApiClient::new(rc.clone())),
            people_api: Box::new(crate::apis::PeopleApiClient::new(rc.clone())),
            reviews_api: Box::new(crate::apis::ReviewsApiClient::new(rc.clone())),
            search_api: Box::new(crate::apis::SearchApiClient::new(rc.clone())),
            trending_api: Box::new(crate::apis::TrendingApiClient::new(rc.clone())),
            tv_api: Box::new(crate::apis::TVApiClient::new(rc.clone())),
            tv_episodes_api: Box::new(crate::apis::TVEpisodesApiClient::new(rc.clone())),
            tv_seasons_api: Box::new(crate::apis::TVSeasonsApiClient::new(rc.clone())),
        }
    }

    pub fn new_with_api_key<T: Into<String>>(api_key: T) -> APIClient {
        let configuration = Configuration::new_with_api_key(api_key);
        APIClient::new(configuration)
    }

    pub fn new_from_env() -> APIClient {
        let api_key = env::var("TMDB_API_KEY").expect("Missing TMDB_API_KEY env var");
        APIClient::new_with_api_key(api_key)
    }

    pub fn account_api(&self) -> &dyn crate::apis::AccountApi {
        self.account_api.as_ref()
    }

    pub fn authentication_api(&self) -> &dyn crate::apis::AuthenticationApi {
        self.authentication_api.as_ref()
    }

    pub fn certifications_api(&self) -> &dyn crate::apis::CertificationsApi {
        self.certifications_api.as_ref()
    }

    pub fn changes_api(&self) -> &dyn crate::apis::ChangesApi {
        self.changes_api.as_ref()
    }

    pub fn collections_api(&self) -> &dyn crate::apis::CollectionsApi {
        self.collections_api.as_ref()
    }

    pub fn companies_api(&self) -> &dyn crate::apis::CompaniesApi {
        self.companies_api.as_ref()
    }

    pub fn configuration_api(&self) -> &dyn crate::apis::ConfigurationApi {
        self.configuration_api.as_ref()
    }

    pub fn credits_api(&self) -> &dyn crate::apis::CreditsApi {
        self.credits_api.as_ref()
    }

    pub fn discover_api(&self) -> &dyn crate::apis::DiscoverApi {
        self.discover_api.as_ref()
    }

    pub fn find_api(&self) -> &dyn crate::apis::FindApi {
        self.find_api.as_ref()
    }

    pub fn genres_api(&self) -> &dyn crate::apis::GenresApi {
        self.genres_api.as_ref()
    }

    pub fn guest_sessions_api(&self) -> &dyn crate::apis::GuestSessionsApi {
        self.guest_sessions_api.as_ref()
    }

    pub fn keywords_api(&self) -> &dyn crate::apis::KeywordsApi {
        self.keywords_api.as_ref()
    }

    pub fn lists_api(&self) -> &dyn crate::apis::ListsApi {
        self.lists_api.as_ref()
    }

    pub fn movies_api(&self) -> &dyn crate::apis::MoviesApi {
        self.movies_api.as_ref()
    }

    pub fn networks_api(&self) -> &dyn crate::apis::NetworksApi {
        self.networks_api.as_ref()
    }

    pub fn people_api(&self) -> &dyn crate::apis::PeopleApi {
        self.people_api.as_ref()
    }

    pub fn reviews_api(&self) -> &dyn crate::apis::ReviewsApi {
        self.reviews_api.as_ref()
    }

    pub fn search_api(&self) -> &dyn crate::apis::SearchApi {
        self.search_api.as_ref()
    }

    pub fn trending_api(&self) -> &dyn crate::apis::TrendingApi {
        self.trending_api.as_ref()
    }

    pub fn tv_api(&self) -> &dyn crate::apis::TVApi {
        self.tv_api.as_ref()
    }

    pub fn tv_episodes_api(&self) -> &dyn crate::apis::TVEpisodesApi {
        self.tv_episodes_api.as_ref()
    }

    pub fn tv_seasons_api(&self) -> &dyn crate::apis::TVSeasonsApi {
        self.tv_seasons_api.as_ref()
    }
}
