use reqwest::{header::HeaderMap, Error};

pub async fn get(url: &str, headers: HeaderMap) -> Result<String, Error> {
    let client = reqwest::Client::new();
    client.get(url).headers(headers).send().await?.text().await
}

pub async fn post(url: &str, headers: HeaderMap, body: Option<&str>) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let data = body.unwrap_or(&"".to_string()).to_string();
    client
        .post(url)
        .headers(headers)
        .body(data)
        .send()
        .await?
        .text()
        .await
}

pub async fn put(url: &str, headers: HeaderMap, body: Option<&str>) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let data = body.unwrap_or(&"".to_string()).to_string();
    client
        .put(url)
        .headers(headers)
        .body(data)
        .send()
        .await?
        .text()
        .await
}

pub async fn delete(url: &str, headers: HeaderMap) -> Result<String, Error> {
    let client = reqwest::Client::new();
    client
        .delete(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await
}

pub async fn patch(url: &str, headers: HeaderMap, body: Option<&str>) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let data = body.unwrap_or(&"".to_string()).to_string();
    client
        .patch(url)
        .headers(headers)
        .body(data)
        .send()
        .await?
        .text()
        .await
}
