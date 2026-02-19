#![cfg(feature = "blocking")]

use tmdb_client::apis::client::APIClient;

#[test]
fn tv_by_id_should_give_results() {
    let client = APIClient::new_from_env();
    // TV ID 1399 is "Game of Thrones"
    let result = client.tv_api().get_tv_details(1399, None, None, None);
    let tv = result.expect("Error querying tv 1399 (Game of Thrones)");

    assert_eq!(tv.name, Some("Game of Thrones".to_owned()));

    // Append lists should not be valuated.
    assert!(tv.credits.is_none());
    assert!(tv.videos.is_none());
    assert!(tv.images.is_none());
    assert!(tv.alternative_titles.is_none());
}

#[test]
fn appends_to_tv_by_id_should_be_valuated() {
    let client = APIClient::new_from_env();
    let result = client.tv_api().get_tv_details(
        1399,
        None,
        None,
        Some("credits,videos,images,alternative_titles"),
    );
    let tv = result.expect("Error querying tv 1399 (Game of Thrones)");

    assert_eq!(tv.name, Some("Game of Thrones".to_owned()));

    // Append lists should be valuated.
    assert!(tv.credits.is_some());
    assert!(tv.videos.is_some());
    assert!(tv.images.is_some());
    assert!(tv.alternative_titles.is_some());
    // Verify the type has 'results' field
    let _ = tv.alternative_titles.unwrap().results;
}
