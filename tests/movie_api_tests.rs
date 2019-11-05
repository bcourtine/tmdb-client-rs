
use tmdb_client::apis::client::APIClient;

#[test]
fn movie_by_id_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.movies_api().get_movie_details(19995, None, None, None);
    let movie = result.expect("Error querying movie 19995 (Avatar)");

    assert_eq!(movie.title, Some("Avatar".to_owned()));

    // Append lists should not be valuated.
    assert!(movie.credits.is_none());
    assert!(movie.videos.is_none());
    assert!(movie.images.is_none());
    assert!(movie.release_dates.is_none());
    assert!(movie.translations.is_none());
    assert!(movie.keywords.is_none());
    assert!(movie.reviews.is_none());
    assert!(movie.external_ids.is_none());
}

#[test]
fn appends_to_movie_by_id_should_be_valuated() {
    let client = APIClient::new_from_env();
    let result = client.movies_api().get_movie_details(
        19995,
        None,
        None,
        Some("credits,videos,images,release_dates,translations,keywords,reviews,external_ids"),
    );
    let movie = result.expect("Error querying movie 19995 (Avatar)");

    assert_eq!(movie.title, Some("Avatar".to_owned()));

    // Append lists should be valuated.
    assert!(movie.credits.is_some());
    assert!(movie.videos.is_some());
    assert!(movie.images.is_some());
    assert!(movie.release_dates.is_some());
    assert!(movie.translations.is_some());
    assert!(movie.keywords.is_some());
    assert!(movie.reviews.is_some());
    assert!(movie.external_ids.is_some());
}

#[test]
fn movie_external_ids_should_be_valuated() {
    let client = APIClient::new_from_env();
    let result = client.movies_api().get_movie_external_ids(19995);
    let external_ids = result.expect("Error querying movie 19995 (Avatar)");

    assert_eq!(external_ids.id, Some(19995));
    assert!(external_ids.imdb_id.is_some());
}
