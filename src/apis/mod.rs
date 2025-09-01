
pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

mod account_api;
pub use self::account_api::{AccountApi, AccountApiClient};
mod authentication_api;
pub use self::authentication_api::{AuthenticationApi, AuthenticationApiClient};
mod certifications_api;
pub use self::certifications_api::{CertificationsApi, CertificationsApiClient};
mod changes_api;
pub use self::changes_api::{ChangesApi, ChangesApiClient};
mod collections_api;
pub use self::collections_api::{CollectionsApi, CollectionsApiClient};
mod companies_api;
pub use self::companies_api::{CompaniesApi, CompaniesApiClient};
mod configuration_api;
pub use self::configuration_api::{ConfigurationApi, ConfigurationApiClient};
mod credits_api;
pub use self::credits_api::{CreditsApi, CreditsApiClient};
mod discover_api;
pub use self::discover_api::{DiscoverApi, DiscoverApiClient};
mod find_api;
pub use self::find_api::{FindApi, FindApiClient};
mod genres_api;
pub use self::genres_api::{GenresApi, GenresApiClient};
mod guest_sessions_api;
pub use self::guest_sessions_api::{GuestSessionsApi, GuestSessionsApiClient};
mod keywords_api;
pub use self::keywords_api::{KeywordsApi, KeywordsApiClient};
mod lists_api;
pub use self::lists_api::{ListsApi, ListsApiClient};
mod movies_api;
pub use self::movies_api::{MoviesApi, MoviesApiClient};
mod networks_api;
pub use self::networks_api::{NetworksApi, NetworksApiClient};
mod people_api;
pub use self::people_api::{PeopleApi, PeopleApiClient};
mod reviews_api;
pub use self::reviews_api::{ReviewsApi, ReviewsApiClient};
mod search_api;
pub use self::search_api::{SearchApi, SearchApiClient};
mod trending_api;
pub use self::trending_api::{TrendingApi, TrendingApiClient};
mod tv_api;
pub use self::tv_api::{TVApi, TVApiClient};
mod tv_episodes_api;
pub use self::tv_episodes_api::{TVEpisodesApi, TVEpisodesApiClient};
mod tv_episode_groups_api;
pub use self::tv_episode_groups_api::{TVEpisodeGroupsApi, TVEpisodeGroupsApiClient};
mod tv_seasons_api;
pub use self::tv_seasons_api::{TVSeasonsApi, TVSeasonsApiClient};

pub mod client;
pub mod configuration;
