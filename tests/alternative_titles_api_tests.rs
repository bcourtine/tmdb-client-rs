
use tmdb_client::apis::client::APIClient;

#[test]
fn tv_alternative_titles_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.tv_api().get_tv_alternative_titles_list(1, None);
    let alternatives = result.expect("Error querying tv alternative titles");
    assert_eq!(alternatives.id, Some(1));
    assert!(alternatives.results.expect("Alternative list unwrap error").len() > 0);
}

#[test]
fn movie_alternative_titles_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.movies_api().get_movie_alternative_titles_list(5, None);
    let alternatives = result.expect("Error querying movie alternative titles");
    assert_eq!(alternatives.id, Some(5));
    assert!(alternatives.titles.expect("Alternative list unwrap error").len() > 0);
}