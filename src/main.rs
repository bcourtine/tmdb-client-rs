extern crate tmdb_client;

use chrono::Local;
use chrono::Duration;
use tmdb_client::files::exports;

pub fn main() {
    let date = Local::now() - Duration::days(1);

    let company_ids = exports::get_production_company_ids(date.naive_local().date());

    println!("{:?}", company_ids.err());
}
