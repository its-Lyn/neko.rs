use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Package {
    #[serde(rename = "Description")]
    pub description: Option<String>,

    #[serde(rename = "FirstSubmitted")]
    pub first_submitted: i32,

    #[serde(rename = "ID")]
    pub id: i32,

    #[serde(rename = "LastModified")]
    pub last_modified: i32,

    #[serde(rename = "Maintainer")]
    pub maintainer: Option<String>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "NumVotes")]
    pub num_votes: i32,

    #[serde(rename = "OutOfDate")]
    pub out_of_date: Option<i32>,

    #[serde(rename = "PackageBase")]
    pub package_base: String,

    #[serde(rename = "PackageBaseID")]
    pub package_base_id: i32,

    #[serde(rename = "Popularity")]
    pub popularity: f32,

    #[serde(rename = "URL")]
    pub url: Option<String>,

    #[serde(rename = "URLPath")]
    pub url_path: Option<String>,

    #[serde(rename = "Version")]
    pub version: String,
}