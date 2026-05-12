use serde::Deserialize;

#[derive(Debug, Clone, Default)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub description: String,
    pub is_installed: bool,
    pub repository: String,
    pub votes: Option<u32>,
    pub popularity: Option<f64>,
    pub out_of_date: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Tab {
    #[default]
    Installed,
    Updates,
}

#[derive(Debug, Deserialize)]
pub struct AurSearchResponse {
    pub results: Vec<AurPackageRaw>,
}

#[derive(Debug, Deserialize)]
pub struct AurPackageRaw {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "NumVotes")]
    pub num_votes: Option<u32>,
    #[serde(rename = "Popularity")]
    pub popularity: Option<f64>,
    #[serde(rename = "OutOfDate")]
    pub out_of_date: Option<u64>,
}
