
use tmdb_client::apis::client::APIClient;

#[test]
fn jobs_list_should_give_results() {
    let client = APIClient::new_from_env();
    let result = client.jobs_api().get_jobs_list();
    let jobs = result.expect("Error querying jobs list API");
    let departements = jobs.jobs.expect("Job list should not be empty");
    assert!(departements.len() >= 1);

    let first_dep = &departements[0];
    // Department should be valuated.
    assert!(first_dep.department.is_some());
    // Department job list should be valuated.
    assert!(first_dep.jobs.is_some());
    if let Some(ref j) = first_dep.jobs {
        assert!(j.len() >= 1);
    }
}
