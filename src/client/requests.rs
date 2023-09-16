use std::error::Error;

pub async fn get_generic(url: &str) -> Result<String, Box<dyn Error>> {
    let body = reqwest::get(url)
        .await?
        .text()
        .await?;

    Ok(body)
}