use std::env;
use std::sync::Arc;

use super::configuration::Configuration;

/// Async TMDB API Client
///
/// This is the main entry point for making async requests to the TMDB API.
/// It provides convenient access to all API endpoints through dedicated client objects.
///
/// # Examples
///
/// ## Creating a client with an API key
///
/// ```no_run
/// # #[cfg(feature = "async")]
/// # {
/// use tmdb_client::async_apis::AsyncAPIClient;
///
/// #[tokio::main]
/// async fn main() {
///     let client = AsyncAPIClient::new_with_api_key("your_api_key_here");
///     let popular_movies = client.movies_api()
///         .get_movie_popular_paginated(None, None, None)
///         .await
///         .unwrap();
///     println!("Found {} popular movies", popular_movies.results.len());
/// }
/// # }
/// ```
///
/// ## Creating a client from environment variable
///
/// ```no_run
/// # #[cfg(feature = "async")]
/// # {
/// use tmdb_client::async_apis::AsyncAPIClient;
///
/// #[tokio::main]
/// async fn main() {
///     // Reads API key from TMDB_API_KEY environment variable
///     let client = AsyncAPIClient::new_from_env();
///
///     let movie = client.movies_api()
///         .get_movie_details(550, None, None, None)
///         .await
///         .unwrap();
///
///     println!("Movie title: {}", movie.title.unwrap_or_default());
/// }
/// # }
/// ```
pub struct AsyncAPIClient {
    account_api: Box<dyn crate::async_apis::AccountApi>,
    authentication_api: Box<dyn crate::async_apis::AuthenticationApi>,
    certifications_api: Box<dyn crate::async_apis::CertificationsApi>,
    changes_api: Box<dyn crate::async_apis::ChangesApi>,
    collections_api: Box<dyn crate::async_apis::CollectionsApi>,
    companies_api: Box<dyn crate::async_apis::CompaniesApi>,
    configuration_api: Box<dyn crate::async_apis::ConfigurationApi>,
    credits_api: Box<dyn crate::async_apis::CreditsApi>,
    discover_api: Box<dyn crate::async_apis::DiscoverApi>,
    find_api: Box<dyn crate::async_apis::FindApi>,
    genres_api: Box<dyn crate::async_apis::GenresApi>,
    guest_sessions_api: Box<dyn crate::async_apis::GuestSessionsApi>,
    keywords_api: Box<dyn crate::async_apis::KeywordsApi>,
    lists_api: Box<dyn crate::async_apis::ListsApi>,
    movies_api: Box<dyn crate::async_apis::MoviesApi>,
    networks_api: Box<dyn crate::async_apis::NetworksApi>,
    people_api: Box<dyn crate::async_apis::PeopleApi>,
    reviews_api: Box<dyn crate::async_apis::ReviewsApi>,
    search_api: Box<dyn crate::async_apis::SearchApi>,
    trending_api: Box<dyn crate::async_apis::TrendingApi>,
    tv_api: Box<dyn crate::async_apis::TVApi>,
    tv_episodes_api: Box<dyn crate::async_apis::TVEpisodesApi>,
    tv_seasons_api: Box<dyn crate::async_apis::TVSeasonsApi>,
    tv_episode_groups_api: Box<dyn crate::async_apis::TVEpisodeGroupsApi>,
}

impl AsyncAPIClient {
    /// Creates a new async API client with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `configuration` - The configuration containing base URL, API key, etc.
    pub fn new(configuration: Configuration) -> AsyncAPIClient {
        let arc = Arc::new(configuration);

        AsyncAPIClient {
            account_api: Box::new(crate::async_apis::AccountApiClient::new(arc.clone())),
            authentication_api: Box::new(crate::async_apis::AuthenticationApiClient::new(
                arc.clone(),
            )),
            certifications_api: Box::new(crate::async_apis::CertificationsApiClient::new(
                arc.clone(),
            )),
            changes_api: Box::new(crate::async_apis::ChangesApiClient::new(arc.clone())),
            collections_api: Box::new(crate::async_apis::CollectionsApiClient::new(arc.clone())),
            companies_api: Box::new(crate::async_apis::CompaniesApiClient::new(arc.clone())),
            configuration_api: Box::new(crate::async_apis::ConfigurationApiClient::new(
                arc.clone(),
            )),
            credits_api: Box::new(crate::async_apis::CreditsApiClient::new(arc.clone())),
            discover_api: Box::new(crate::async_apis::DiscoverApiClient::new(arc.clone())),
            find_api: Box::new(crate::async_apis::FindApiClient::new(arc.clone())),
            genres_api: Box::new(crate::async_apis::GenresApiClient::new(arc.clone())),
            guest_sessions_api: Box::new(crate::async_apis::GuestSessionsApiClient::new(
                arc.clone(),
            )),
            keywords_api: Box::new(crate::async_apis::KeywordsApiClient::new(arc.clone())),
            lists_api: Box::new(crate::async_apis::ListsApiClient::new(arc.clone())),
            movies_api: Box::new(crate::async_apis::MoviesApiClient::new(arc.clone())),
            networks_api: Box::new(crate::async_apis::NetworksApiClient::new(arc.clone())),
            people_api: Box::new(crate::async_apis::PeopleApiClient::new(arc.clone())),
            reviews_api: Box::new(crate::async_apis::ReviewsApiClient::new(arc.clone())),
            search_api: Box::new(crate::async_apis::SearchApiClient::new(arc.clone())),
            trending_api: Box::new(crate::async_apis::TrendingApiClient::new(arc.clone())),
            tv_api: Box::new(crate::async_apis::TVApiClient::new(arc.clone())),
            tv_episodes_api: Box::new(crate::async_apis::TVEpisodesApiClient::new(arc.clone())),
            tv_seasons_api: Box::new(crate::async_apis::TVSeasonsApiClient::new(arc.clone())),
            tv_episode_groups_api: Box::new(crate::async_apis::TVEpisodeGroupsApiClient::new(
                arc.clone(),
            )),
        }
    }

    /// Creates a new async API client with the given API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` - Your TMDB API key (v3)
    pub fn new_with_api_key<T: Into<String>>(api_key: T) -> AsyncAPIClient {
        let configuration = Configuration::new_with_api_key(api_key);
        AsyncAPIClient::new(configuration)
    }

    /// Creates a new async API client reading the API key from the `TMDB_API_KEY` environment variable.
    ///
    /// # Panics
    ///
    /// Panics if the `TMDB_API_KEY` environment variable is not set.
    pub fn new_from_env() -> AsyncAPIClient {
        let api_key = env::var("TMDB_API_KEY").expect("Missing TMDB_API_KEY env var");
        AsyncAPIClient::new_with_api_key(api_key)
    }

    // Accessor methods for all APIs

    pub fn account_api(&self) -> &(dyn crate::async_apis::AccountApi) {
        self.account_api.as_ref()
    }

    pub fn authentication_api(&self) -> &(dyn crate::async_apis::AuthenticationApi) {
        self.authentication_api.as_ref()
    }

    pub fn certifications_api(&self) -> &(dyn crate::async_apis::CertificationsApi) {
        self.certifications_api.as_ref()
    }

    pub fn changes_api(&self) -> &(dyn crate::async_apis::ChangesApi) {
        self.changes_api.as_ref()
    }

    pub fn collections_api(&self) -> &(dyn crate::async_apis::CollectionsApi) {
        self.collections_api.as_ref()
    }

    pub fn companies_api(&self) -> &(dyn crate::async_apis::CompaniesApi) {
        self.companies_api.as_ref()
    }

    pub fn configuration_api(&self) -> &(dyn crate::async_apis::ConfigurationApi) {
        self.configuration_api.as_ref()
    }

    pub fn credits_api(&self) -> &(dyn crate::async_apis::CreditsApi) {
        self.credits_api.as_ref()
    }

    pub fn discover_api(&self) -> &(dyn crate::async_apis::DiscoverApi) {
        self.discover_api.as_ref()
    }

    pub fn find_api(&self) -> &(dyn crate::async_apis::FindApi) {
        self.find_api.as_ref()
    }

    pub fn genres_api(&self) -> &(dyn crate::async_apis::GenresApi) {
        self.genres_api.as_ref()
    }

    pub fn guest_sessions_api(&self) -> &(dyn crate::async_apis::GuestSessionsApi) {
        self.guest_sessions_api.as_ref()
    }

    pub fn keywords_api(&self) -> &(dyn crate::async_apis::KeywordsApi) {
        self.keywords_api.as_ref()
    }

    pub fn lists_api(&self) -> &(dyn crate::async_apis::ListsApi) {
        self.lists_api.as_ref()
    }

    pub fn movies_api(&self) -> &(dyn crate::async_apis::MoviesApi) {
        self.movies_api.as_ref()
    }

    pub fn networks_api(&self) -> &(dyn crate::async_apis::NetworksApi) {
        self.networks_api.as_ref()
    }

    pub fn people_api(&self) -> &(dyn crate::async_apis::PeopleApi) {
        self.people_api.as_ref()
    }

    pub fn reviews_api(&self) -> &(dyn crate::async_apis::ReviewsApi) {
        self.reviews_api.as_ref()
    }

    pub fn search_api(&self) -> &(dyn crate::async_apis::SearchApi) {
        self.search_api.as_ref()
    }

    pub fn trending_api(&self) -> &(dyn crate::async_apis::TrendingApi) {
        self.trending_api.as_ref()
    }

    pub fn tv_api(&self) -> &(dyn crate::async_apis::TVApi) {
        self.tv_api.as_ref()
    }

    pub fn tv_episodes_api(&self) -> &(dyn crate::async_apis::TVEpisodesApi) {
        self.tv_episodes_api.as_ref()
    }

    pub fn tv_seasons_api(&self) -> &(dyn crate::async_apis::TVSeasonsApi) {
        self.tv_seasons_api.as_ref()
    }

    pub fn tv_episode_groups_api(&self) -> &(dyn crate::async_apis::TVEpisodeGroupsApi) {
        self.tv_episode_groups_api.as_ref()
    }
}
