
use tmdb_client::apis::client::APIClient;

#[test]
fn search_movie_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.search_api().get_search_movie_paginated("Captain Marvel", 2019, 2019, "fr-FR", 1, true, "FR");
    let movies_paginated = result.unwrap();
    assert!(movies_paginated.total_results.unwrap() >= 1);
}

#[test]
fn search_person_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.search_api().get_search_person_paginated("Julia Roberts", "fr-FR", 1, true, "FR");
    let persons_paginated = result.unwrap();
    assert!(persons_paginated.total_results.unwrap() >= 1);
}
