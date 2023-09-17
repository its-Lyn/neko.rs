use crate::client::requests::get_generic;
use crate::models::package_results::PackageResults;

pub async fn get_aur_packages(package_name: &str) -> PackageResults {
    let aur_url = format!("https://aur.archlinux.org/rpc/?v=5&type=search&arg={}", package_name);
    let json_str: String = get_generic(aur_url.as_str()).await.unwrap();

    serde_json::from_str(json_str.as_str()).unwrap()
}