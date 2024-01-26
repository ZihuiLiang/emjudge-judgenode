use emjudge_judgenode::settings::Settings;
use reqwest::{header::{HeaderMap, HeaderValue}, Error};

#[tokio::test]
async fn is_authed() -> Result<(), Error> {
    let settings = Settings::load_from_file("config/settings.toml", config::FileFormat::Toml).unwrap();
    let license = settings.license;
    let mut headers = HeaderMap::new();
    let url = format!("http://{}:{}/is_authed", settings.url, settings.port);
    let url = url.as_str();
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 401);
    assert_eq!(response.text().await.unwrap(), "Unauthorized");
    headers.insert("Authorization", HeaderValue::from_str("Bearer xx").unwrap());
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 401);
    assert_eq!(response.text().await.unwrap(), "Unauthorized");
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await.unwrap(), "Authorized");
    Ok(())
}