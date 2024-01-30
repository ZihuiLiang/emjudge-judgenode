use std::io::Read;

use emjudge_judgecore::quantity::{MemorySize, TimeSpan};
use emjudge_judgenode::{data::TestData, requests::TestRequest, responses::OnlyRunResponse, settings::Settings};
use reqwest::{header::{HeaderMap, HeaderValue}, Error};

#[tokio::test]
async fn hello_world() -> Result<(), Error> {
    let settings = Settings::load_from_file("config/settings.toml", config::FileFormat::Toml).unwrap();
    let license = settings.license;
    let mut headers = HeaderMap::new();
    let url = format!("http://{}:{}/test_data/submit", settings.url, settings.port);
    let url = url.as_str();
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let mut tested_code = vec![];
    std::fs::File::open("tests/programs/helloworld/main.cpp").unwrap().read_to_end(&mut tested_code).unwrap();
    let test_data = TestData {
        id: String::from("0"),
        input: vec![vec![];100],
        output: vec![vec![];100],
        time_limit: settings.max_time_limit,
        memory_limit: settings.max_memory_limit,
        eval_or_interactor_code: vec![],
        eval_or_interactor_language: String::from("C++"),
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
    };
    let response = reqwest::Client::new()
        .post(url)
        .json(&test_data)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);    
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::Ok).unwrap());
    let url = format!("http://{}:{}/test/only_run", settings.url, settings.port);
    let url = url.as_str();
    let test_request = TestRequest {
        test_uuid: String::from("0"),
        code: tested_code.clone(),
        language: String::from("C++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .json(&test_request)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let result: Vec<OnlyRunResponse> = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(result.len(), 100);
    for i in result {
        match i {
            OnlyRunResponse::Ok(result) => {
                assert_eq!(result.stdout, "Hello, World!\n".as_bytes());
            },
            _ => panic!(),
        }
    }
    Ok(())
}   

#[tokio::test]
async fn mle() -> Result<(), Error> {
    let settings = Settings::load_from_file("config/settings.toml", config::FileFormat::Toml).unwrap();
    let license = settings.license;
    let mut headers = HeaderMap::new();
    let url = format!("http://{}:{}/test_data/submit", settings.url, settings.port);
    let url = url.as_str();
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let mut tested_code = vec![];
    std::fs::File::open("tests/programs/mle.cpp").unwrap().read_to_end(&mut tested_code).unwrap();
    let test_data = TestData {
        id: String::from("1"),
        input: vec![vec![];100],
        output: vec![vec![];100],
        time_limit: TimeSpan::from_seconds(1),
        memory_limit: MemorySize::from_megabytes(1),
        eval_or_interactor_code: vec![],
        eval_or_interactor_language: String::from("C++"),
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
    };
    let response = reqwest::Client::new()
        .post(url)
        .json(&test_data)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);    
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::Ok).unwrap());
    let url = format!("http://{}:{}/test/only_run", settings.url, settings.port);
    let url = url.as_str();
    let test_request = TestRequest {
        test_uuid: String::from("1"),
        code: tested_code.clone(),
        language: String::from("C++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .json(&test_request)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    let result: Vec<OnlyRunResponse> = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(result.len(), 100);
    for i in result {
        match i {
            OnlyRunResponse::MemoryLimitExceeded(result) => {
                assert!(result.memory >= MemorySize::from_megabytes(1));
            },
            i => panic!("{:?}", i),
        }
    }
    Ok(())
}