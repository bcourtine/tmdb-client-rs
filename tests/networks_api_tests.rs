
use tmdb_client::apis::client::APIClient;

#[test]
fn network_alternative_names_list_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.networks_api().get_network_alternative_names_list(1);
    let alternatives = result.expect("Error querying network");
    assert_eq!(alternatives.id, Some(1));
    assert!(alternatives.results.expect("Alternative result list unwrap error").len() > 0);
}


#[test]
fn network_images_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.networks_api().get_network_images(1);
    let images = result.expect("Error querying network");
    assert_eq!(images.id, Some(1));
    assert!(images.logos.expect("Network logo list unwrap error").len() > 0);
}
