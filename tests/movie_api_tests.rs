
use tmdb_client::apis::client::APIClient;

#[test]
fn movie_by_id_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.movies_api().get_movie_details(19995, None, None);
    let movie = result.expect("Error querying movie 19995 (Avatar)");

    assert_eq!(movie.title, Some("Avatar".to_owned()));

    // Append lists should not be valuated.
    assert!(movie.credits.is_none());
    assert!(movie.videos.is_none());
    assert!(movie.images.is_none());
}

#[test]
fn appends_to_movie_by_id_should_be_valuated() {
    let client = APIClient::new_from_env();
    let result = client.movies_api().get_movie_details(
        19995,
        None,
        Some("credits,videos,images")
    );
    let movie = result.expect("Error querying movie 19995 (Avatar)");

    assert_eq!(movie.title, Some("Avatar".to_owned()));

    // Append lists should be valuated.
    assert!(movie.credits.is_some());
    assert!(movie.videos.is_some());
    assert!(movie.images.is_some());
}