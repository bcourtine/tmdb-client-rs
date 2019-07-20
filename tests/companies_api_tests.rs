
use tmdb_client::apis::client::APIClient;

#[test]
fn companies_images_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.companies_api().get_company_images(670);
    let images = result.expect("Error querying James Bond collection");
    assert_eq!(images.id, Some(670));
    assert!(images.logos.expect("Logos unwrap error").len() > 0);
    assert!(images.posters.is_none());
    assert!(images.backdrops.is_none());
}
