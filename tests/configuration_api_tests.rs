
use tmdb_client::apis::client::APIClient;
use std::time::{Instant, Duration};

#[test]
fn rate_limit_client_should_work() {
    let start = Instant::now();
    let client = APIClient::new_from_env();
    for _i in 0..8 {
        client.configuration_api().get_configuration(None).expect("Error querying configuration");
    }
    assert!(start.elapsed() >= Duration::from_secs(2));
}


#[test]
fn jobs_list_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.configuration_api().get_jobs_list();
    let jobs = result.expect("Error querying jobs list API");
    assert!(jobs.len() >= 1);

    let first_dep = &jobs[0];
    // Department should be valuated.
    assert!(first_dep.department.is_some());
    // Department job list should be valuated.
    assert!(first_dep.jobs.is_some());
    if let Some(ref j) = first_dep.jobs {
        assert!(j.len() >= 1);
    }
}

#[test]
fn timezones_list_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.configuration_api().get_timezones_list();
    let timezones = result.expect("Error querying timezones list API");
    assert!(timezones.len() >= 1);

    let first_tz = &timezones[0];
    assert!(first_tz.iso_3166_1.is_some());
    assert!(first_tz.zones.is_some());
    if let Some(ref z) = first_tz.zones {
        assert!(z.len() >= 1);
    }
}

#[test]
fn countries_list_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.configuration_api().get_countries_list();
    let translations = result.expect("Error querying countries list API");
    assert!(translations.len() >= 1);

    let first_country = &translations[0];
    assert!(first_country.iso_3166_1.is_some());
    assert!(first_country.english_name.is_some());
    assert!(first_country.iso_639_1.is_none());
}

#[test]
fn languages_list_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.configuration_api().get_languages_list();
    let languages = result.expect("Error querying languages list API");
    assert!(languages.len() >= 1);

    let first_lang = &languages[0];
    assert!(first_lang.iso_639_1.is_some());
    assert!(first_lang.name.is_some());
    assert!(first_lang.english_name.is_some());

}

#[test]
fn primary_translations_list_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.configuration_api().get_primary_translations_list();
    let primary_translations = result.expect("Error querying primary translations list API");
    assert!(primary_translations.len() >= 1);
}
