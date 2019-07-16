
use tmdb_client::apis::client::APIClient;

#[test]
fn people_translations_list_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.people_api().get_person_translations_list(1234, None);
    let translations = result.expect("Error querying people translations");
    assert_eq!(translations.id, Some(1234));
    assert!(translations.translations.expect("Translation list unwrap error").len() > 0);
}
