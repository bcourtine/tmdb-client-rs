
use tmdb_client::apis::client::APIClient;
use tmdb_client::models::{MediaType, TimeWindow};

#[test]
fn trending_movies_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.trending_api().get_trending_paginated(&MediaType::Movie, &TimeWindow::Week);
    let trending = result.expect("Error querying trending");
    assert!(trending.results.is_some());
    assert!(trending.results.expect("Trending list unwrap error").len() > 0);
}
