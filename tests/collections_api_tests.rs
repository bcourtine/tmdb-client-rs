
use tmdb_client::apis::client::APIClient;

#[test]
fn collection_translations_list_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.collections_api().get_collection_translations_list(645, None);
    let translations = result.expect("Error querying James Bond collection");
    assert_eq!(translations.id, Some(645));
    assert!(translations.translations.expect("Translation list unwrap error").len() > 0);
}
