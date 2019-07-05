
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
    let timezones = result.expect("Error querying jobs list API");
    assert!(timezones.len() >= 1);

    let first_tz = &timezones[0];
    // Department should be valuated.
    assert!(first_tz.iso_3166_1.is_some());
    // Department job list should be valuated.
    assert!(first_tz.zones.is_some());
    if let Some(ref z) = first_tz.zones {
        assert!(z.len() >= 1);
    }
}
