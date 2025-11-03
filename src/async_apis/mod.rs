/*
 * Async API module
 *
 * This module provides async versions of all TMDB API clients.
 * It reuses the same models from the parent module but provides
 * async implementations using tokio and async-trait.
 */

// Configuration
pub mod configuration;

// All API modules
pub mod account_api;
pub mod authentication_api;
pub mod certifications_api;
pub mod changes_api;
pub mod collections_api;
pub mod companies_api;
pub mod configuration_api;
pub mod credits_api;
pub mod discover_api;
pub mod find_api;
pub mod genres_api;
pub mod guest_sessions_api;
pub mod keywords_api;
pub mod lists_api;
pub mod movies_api;
pub mod networks_api;
pub mod people_api;
pub mod reviews_api;
pub mod search_api;
pub mod trending_api;
pub mod tv_api;
pub mod tv_episode_groups_api;
pub mod tv_episodes_api;
pub mod tv_seasons_api;

// Client facade
pub mod client;

// Re-exports for convenience
pub use configuration::Configuration;
pub use client::AsyncAPIClient;

// Re-export all API traits
pub use account_api::{AccountApi, AccountApiClient};
pub use authentication_api::{AuthenticationApi, AuthenticationApiClient};
pub use certifications_api::{CertificationsApi, CertificationsApiClient};
pub use changes_api::{ChangesApi, ChangesApiClient};
pub use collections_api::{CollectionsApi, CollectionsApiClient};
pub use companies_api::{CompaniesApi, CompaniesApiClient};
pub use configuration_api::{ConfigurationApi, ConfigurationApiClient};
pub use credits_api::{CreditsApi, CreditsApiClient};
pub use discover_api::{DiscoverApi, DiscoverApiClient};
pub use find_api::{FindApi, FindApiClient};
pub use genres_api::{GenresApi, GenresApiClient};
pub use guest_sessions_api::{GuestSessionsApi, GuestSessionsApiClient};
pub use keywords_api::{KeywordsApi, KeywordsApiClient};
pub use lists_api::{ListsApi, ListsApiClient};
pub use movies_api::{MoviesApi, MoviesApiClient};
pub use networks_api::{NetworksApi, NetworksApiClient};
pub use people_api::{PeopleApi, PeopleApiClient};
pub use reviews_api::{ReviewsApi, ReviewsApiClient};
pub use search_api::{SearchApi, SearchApiClient};
pub use trending_api::{TrendingApi, TrendingApiClient};
pub use tv_api::{TVApi, TVApiClient};
pub use tv_episode_groups_api::{TVEpisodeGroupsApi, TVEpisodeGroupsApiClient};
pub use tv_episodes_api::{TVEpisodesApi, TVEpisodesApiClient};
pub use tv_seasons_api::{TVSeasonsApi, TVSeasonsApiClient};

// Helper function for URL encoding (shared with sync implementation)
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}
