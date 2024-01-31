use emjudge_judgenode::settings::Settings;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let settings =
        Settings::load_from_file("config/settings.toml", config::FileFormat::Toml).unwrap();
    let license = settings.license;
    let mut headers = HeaderMap::new();
    let url = format!(
        "http://{}:{}/info/test_pressure",
        settings.url, settings.port
    );
    headers.insert(
        "Authorization",
        HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap(),
    );
    loop {
        let response = reqwest::Client::new()
            .get(url.clone())
            .headers(headers.clone())
            .send()
            .await
            .unwrap();
        assert_eq!(response.status().as_u16(), 200);
        let result: emjudge_judgenode::responses::TestPressureResponse =
            serde_json::from_str(&response.text().await.unwrap()).unwrap();
        println!("{:?}", result);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
