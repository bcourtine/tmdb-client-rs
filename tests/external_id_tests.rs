use chrono::Duration;
use chrono::Local;

use tmdb_client::files::exports;

#[test]
fn collection_ids_should_be_downloaded_and_parsed() {
    let date = Local::now() - Duration::days(1);
    let exported_ids = exports::get_collection_ids(date.naive_local().date());
    assert!(exported_ids.expect("File download or deserialization error").len() > 0);
}

#[test]
fn keyword_ids_should_be_downloaded_and_parsed() {
    let date = Local::now() - Duration::days(1);
    let exported_ids = exports::get_collection_ids(date.naive_local().date());
    assert!(exported_ids.expect("File download or deserialization error").len() > 0);
}


#[test]
fn production_company_ids_should_be_downloaded_and_parsed() {
    let date = Local::now() - Duration::days(1);
    let exported_ids = exports::get_production_company_ids(date.naive_local().date());
    assert!(exported_ids.expect("File download or deserialization error").len() > 0);
}

#[test]
fn movie_ids_should_be_downloaded_and_parsed() {
    let date = Local::now() - Duration::days(1);
    let exported_ids = exports::get_movie_ids(date.naive_local().date());
    assert!(exported_ids.expect("File download or deserialization error").len() > 0);
}

#[test]
fn tv_series_ids_should_be_downloaded_and_parsed() {
    let date = Local::now() - Duration::days(1);
    let exported_ids = exports::get_tv_series_ids(date.naive_local().date());
    assert!(exported_ids.expect("File download or deserialization error").len() > 0);
}

#[test]
fn tv_network_ids_should_be_downloaded_and_parsed() {
    let date = Local::now() - Duration::days(1);
    let exported_ids = exports::get_tv_network_ids(date.naive_local().date());
    assert!(exported_ids.expect("File download or deserialization error").len() > 0);
}

#[test]
fn person_ids_should_be_downloaded_and_parsed() {
    let date = Local::now() - Duration::days(1);
    let exported_ids = exports::get_person_ids(date.naive_local().date());
    assert!(exported_ids.expect("File download or deserialization error").len() > 0);
}
