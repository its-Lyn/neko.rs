use serde::Deserialize;
use crate::models::package::Package;

#[derive(Deserialize, Debug)]
pub struct PackageResults {
    #[serde(rename = "resultcount")]
    pub result_count: i32,

    pub results: Vec<Package>,

    #[serde(rename = "type")]
    pub s_type: String,

    pub version: i32
}