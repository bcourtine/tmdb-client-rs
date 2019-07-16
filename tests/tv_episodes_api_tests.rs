
use tmdb_client::apis::client::APIClient;

#[test]
fn tv_episode_translations_list_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.tv_episodes_api().get_tv_season_episode_translations_list(12345, 1, 1);
    let translations = result.expect("Error querying tv episode translations");
    assert_eq!(translations.id, Some(479529));
    assert!(translations.translations.expect("Translation list unwrap error").len() > 0);
}
