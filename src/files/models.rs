
#[derive(Debug, Deserialize)]
pub struct ProductionCompanyExportedId {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct KeywordExportedId {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CollectionExportedId {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct TvNetworkExportedId {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct MovieExportedId {
    pub id: i64,
    pub original_title: String,
    pub popularity: f64,
    pub adult: bool,
    pub video: bool,
}

#[derive(Debug, Deserialize)]
pub struct TvSeriesExportedId {
    pub id: i64,
    pub original_name: String,
    pub popularity: f64,
}

#[derive(Debug, Deserialize)]
pub struct PersonExportedId {
    pub id: i64,
    pub name: String,
    pub popularity: f64,
    pub adult: bool,
}
