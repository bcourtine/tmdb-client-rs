use chrono::NaiveDate;
use crate::Error;
use flate2::read::GzDecoder;
use crate::files::models::{ProductionCompanyExportedId, KeywordExportedId, TvNetworkExportedId, CollectionExportedId, PersonExportedId, TvSeriesExportedId, MovieExportedId};
use std::io::BufReader;

const EXPORT_BASE_URL: &str = "http://files.tmdb.org/p/exports";
const EXPORT_DATE_FORMAT: &str = "%m_%d_%Y";

pub fn get_production_company_ids(export_date: NaiveDate) -> Result<Vec<ProductionCompanyExportedId>, Error> {
    get_ids("production_company", export_date)
}

pub fn get_keyword_ids(export_date: NaiveDate) -> Result<Vec<KeywordExportedId>, Error> {
    get_ids("keyword", export_date)
}

pub fn get_tv_network_ids(export_date: NaiveDate) -> Result<Vec<TvNetworkExportedId>, Error> {
    get_ids("tv_network", export_date)
}

pub fn get_collection_ids(export_date: NaiveDate) -> Result<Vec<CollectionExportedId>, Error> {
    get_ids("collection", export_date)
}

pub fn get_person_ids(export_date: NaiveDate) -> Result<Vec<PersonExportedId>, Error> {
    get_ids("person", export_date)
}

pub fn get_tv_series_ids(export_date: NaiveDate) -> Result<Vec<TvSeriesExportedId>, Error> {
    get_ids("tv_series", export_date)
}

pub fn get_movie_ids(export_date: NaiveDate) -> Result<Vec<MovieExportedId>, Error> {
    get_ids("movie", export_date)
}

pub fn get_ids<T>(export_type: impl AsRef<str>, export_date: NaiveDate) -> Result<Vec<T>, Error>
    where T: serde::de::DeserializeOwned {

    let file_date = export_date.format(EXPORT_DATE_FORMAT);
    let file_name = format!("{}_ids_{}.json.gz", export_type.as_ref(), file_date);
    let url = format!("{}/{}", EXPORT_BASE_URL, &file_name);

    let response = reqwest::get(&url)?;
    let response_buffered = BufReader::new(response);
    let uncompressed_content = GzDecoder::new(response_buffered);
    let uncompressed_content_buffered = BufReader::new(uncompressed_content);

    let res: Vec<T> = serde_json::Deserializer::from_reader(uncompressed_content_buffered)
        .into_iter()
        .filter_map(|r| r.ok())
        .collect();

    Ok(res)
}
