use std::rc::Rc;

use super::configuration::Configuration;

pub struct APIClient {
    configuration: Rc<Configuration>,
    account_api: Box<::apis::AccountApi>,
    authentication_api: Box<::apis::AuthenticationApi>,
    certifications_api: Box<::apis::CertificationsApi>,
    changes_api: Box<::apis::ChangesApi>,
    collections_api: Box<::apis::CollectionsApi>,
    companies_api: Box<::apis::CompaniesApi>,
    configuration_api: Box<::apis::ConfigurationApi>,
    credits_api: Box<::apis::CreditsApi>,
    discover_api: Box<::apis::DiscoverApi>,
    find_api: Box<::apis::FindApi>,
    genres_api: Box<::apis::GenresApi>,
    guest_sessions_api: Box<::apis::GuestSessionsApi>,
    jobs_api: Box<::apis::JobsApi>,
    keywords_api: Box<::apis::KeywordsApi>,
    lists_api: Box<::apis::ListsApi>,
    movies_api: Box<::apis::MoviesApi>,
    networks_api: Box<::apis::NetworksApi>,
    people_api: Box<::apis::PeopleApi>,
    reviews_api: Box<::apis::ReviewsApi>,
    search_api: Box<::apis::SearchApi>,
    tv_api: Box<::apis::TVApi>,
    tv_episodes_api: Box<::apis::TVEpisodesApi>,
    tv_seasons_api: Box<::apis::TVSeasonsApi>,
    timezones_api: Box<::apis::TimezonesApi>,
}

impl APIClient {
    pub fn new(configuration: Configuration) -> APIClient {
        let rc = Rc::new(configuration);

        APIClient {
            configuration: rc.clone(),
            account_api: Box::new(::apis::AccountApiClient::new(rc.clone())),
            authentication_api: Box::new(::apis::AuthenticationApiClient::new(rc.clone())),
            certifications_api: Box::new(::apis::CertificationsApiClient::new(rc.clone())),
            changes_api: Box::new(::apis::ChangesApiClient::new(rc.clone())),
            collections_api: Box::new(::apis::CollectionsApiClient::new(rc.clone())),
            companies_api: Box::new(::apis::CompaniesApiClient::new(rc.clone())),
            configuration_api: Box::new(::apis::ConfigurationApiClient::new(rc.clone())),
            credits_api: Box::new(::apis::CreditsApiClient::new(rc.clone())),
            discover_api: Box::new(::apis::DiscoverApiClient::new(rc.clone())),
            find_api: Box::new(::apis::FindApiClient::new(rc.clone())),
            genres_api: Box::new(::apis::GenresApiClient::new(rc.clone())),
            guest_sessions_api: Box::new(::apis::GuestSessionsApiClient::new(rc.clone())),
            jobs_api: Box::new(::apis::JobsApiClient::new(rc.clone())),
            keywords_api: Box::new(::apis::KeywordsApiClient::new(rc.clone())),
            lists_api: Box::new(::apis::ListsApiClient::new(rc.clone())),
            movies_api: Box::new(::apis::MoviesApiClient::new(rc.clone())),
            networks_api: Box::new(::apis::NetworksApiClient::new(rc.clone())),
            people_api: Box::new(::apis::PeopleApiClient::new(rc.clone())),
            reviews_api: Box::new(::apis::ReviewsApiClient::new(rc.clone())),
            search_api: Box::new(::apis::SearchApiClient::new(rc.clone())),
            tv_api: Box::new(::apis::TVApiClient::new(rc.clone())),
            tv_episodes_api: Box::new(::apis::TVEpisodesApiClient::new(rc.clone())),
            tv_seasons_api: Box::new(::apis::TVSeasonsApiClient::new(rc.clone())),
            timezones_api: Box::new(::apis::TimezonesApiClient::new(rc.clone())),
        }
    }

    pub fn account_api(&self) -> &::apis::AccountApi{
        self.account_api.as_ref()
    }

    pub fn authentication_api(&self) -> &::apis::AuthenticationApi{
        self.authentication_api.as_ref()
    }

    pub fn certifications_api(&self) -> &::apis::CertificationsApi{
        self.certifications_api.as_ref()
    }

    pub fn changes_api(&self) -> &::apis::ChangesApi{
        self.changes_api.as_ref()
    }

    pub fn collections_api(&self) -> &::apis::CollectionsApi{
        self.collections_api.as_ref()
    }

    pub fn companies_api(&self) -> &::apis::CompaniesApi{
        self.companies_api.as_ref()
    }

    pub fn configuration_api(&self) -> &::apis::ConfigurationApi{
        self.configuration_api.as_ref()
    }

    pub fn credits_api(&self) -> &::apis::CreditsApi{
        self.credits_api.as_ref()
    }

    pub fn discover_api(&self) -> &::apis::DiscoverApi{
        self.discover_api.as_ref()
    }

    pub fn find_api(&self) -> &::apis::FindApi{
        self.find_api.as_ref()
    }

    pub fn genres_api(&self) -> &::apis::GenresApi{
        self.genres_api.as_ref()
    }

    pub fn guest_sessions_api(&self) -> &::apis::GuestSessionsApi{
        self.guest_sessions_api.as_ref()
    }

    pub fn jobs_api(&self) -> &::apis::JobsApi{
        self.jobs_api.as_ref()
    }

    pub fn keywords_api(&self) -> &::apis::KeywordsApi{
        self.keywords_api.as_ref()
    }

    pub fn lists_api(&self) -> &::apis::ListsApi{
        self.lists_api.as_ref()
    }

    pub fn movies_api(&self) -> &::apis::MoviesApi{
        self.movies_api.as_ref()
    }

    pub fn networks_api(&self) -> &::apis::NetworksApi{
        self.networks_api.as_ref()
    }

    pub fn people_api(&self) -> &::apis::PeopleApi{
        self.people_api.as_ref()
    }

    pub fn reviews_api(&self) -> &::apis::ReviewsApi{
        self.reviews_api.as_ref()
    }

    pub fn search_api(&self) -> &::apis::SearchApi{
        self.search_api.as_ref()
    }

    pub fn tv_api(&self) -> &::apis::TVApi{
        self.tv_api.as_ref()
    }

    pub fn tv_episodes_api(&self) -> &::apis::TVEpisodesApi{
        self.tv_episodes_api.as_ref()
    }

    pub fn tv_seasons_api(&self) -> &::apis::TVSeasonsApi{
        self.tv_seasons_api.as_ref()
    }

    pub fn timezones_api(&self) -> &::apis::TimezonesApi{
        self.timezones_api.as_ref()
    }

}
