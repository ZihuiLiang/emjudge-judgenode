use emjudge_judgenode::{requests::LanguageRequest, settings::Settings};
use reqwest::{header::{HeaderMap, HeaderValue}, Error};

#[tokio::test]
async fn resource_info() -> Result<(), Error> {
    let settings = Settings::load_from_file("config/settings.toml", config::FileFormat::Toml).unwrap();
    let license = settings.license;
    let mut headers = HeaderMap::new();
    let url = format!("http://{}:{}/info/all", settings.url, settings.port);
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let result: emjudge_judgenode::responses::AllInfoResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    println!("{:?}", result);
    let url = format!("http://{}:{}/info/disk", settings.url, settings.port);
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let result: emjudge_judgenode::responses::DiskInfoResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    println!("{:?}", result);
    let url = format!("http://{}:{}/info/cpu", settings.url, settings.port);
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let result: emjudge_judgenode::responses::CpuInfoResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    println!("{:?}", result);
    let url = format!("http://{}:{}/info/mem", settings.url, settings.port);
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let result: emjudge_judgenode::responses::MemInfoResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    println!("{:?}", result);
    let url = format!("http://{}:{}/info/sys", settings.url, settings.port);
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let result: emjudge_judgenode::responses::SysInfoResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    println!("{:?}", result);
    Ok(())
}

#[tokio::test]
async fn language_info() -> Result<(), Error>  {
    let settings = Settings::load_from_file("config/settings.toml", config::FileFormat::Toml).unwrap();
    let license = settings.license;
    let mut headers = HeaderMap::new();
    let url = format!("http://{}:{}/info/languages", settings.url, settings.port);
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let result: emjudge_judgenode::responses::LanguagesInfoResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    println!("{:?}", result);
    let url = format!("http://{}:{}/info/language", settings.url, settings.port);
    let request = LanguageRequest {
        language: String::from("C++"),
    };
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let response = reqwest::Client::new()
        .post(url)
        .headers(headers.clone())
        .json(&request)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let result: emjudge_judgenode::responses::LanguageInfoResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    match result {
        emjudge_judgenode::responses::LanguageInfoResponse::Ok(result) => {
            println!("{:?}", result);
        },
        _ => panic!(),
    }
    let request = LanguageRequest {
        language: String::from("D++"),
    };
    let url = format!("http://{}:{}/info/language", settings.url, settings.port);
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let response = reqwest::Client::new()
        .post(url)
        .headers(headers.clone())
        .json(&request)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let result: emjudge_judgenode::responses::LanguageInfoResponse = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    match result {
        emjudge_judgenode::responses::LanguageInfoResponse::Ok(result) => {
            panic!("{:?}", result);
        },
        result => {
            println!("{:?}", result);
        },
    }
    Ok(())
}