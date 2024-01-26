use std::io::Read;

use emjudge_judgecore::quantity::{MemorySize, TimeSpan};
use emjudge_judgenode::{db::TestData, service::tests::{RunAndInteractResponse, TestRequest}, settings::Settings};
use reqwest::{header::{HeaderMap, HeaderValue}, Error};

#[tokio::test]
async fn guessnumber() -> Result<(), Error> {
    let settings = Settings::load_from_file("config/settings.toml", config::FileFormat::Toml).unwrap();
    let license = settings.license;
    let mut headers = HeaderMap::new();
    let url = format!("http://{}:{}/test_data/submit", settings.url, settings.port);
    let url = url.as_str();
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let mut tested_code = vec![];
    let mut interact_code = vec![];
    std::fs::File::open("tests/programs/guessnumber/tested.cpp").unwrap().read_to_end(&mut tested_code).unwrap();
    std::fs::File::open("tests/programs/guessnumber/interactor.cpp").unwrap().read_to_end(&mut interact_code).unwrap();
    let test_data = TestData {
        id: String::from("3"),
        input: vec!["1 100 100\n".as_bytes().to_vec();100],
        output: vec![vec![];100],
        time_limit: TimeSpan::from_seconds(1),
        memory_limit: MemorySize::from_megabytes(32),
        eval_or_interactor_code: interact_code,
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
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::service::test_data::SubmitResult::Ok).unwrap());
    let url = format!("http://{}:{}/test/run_and_interact", settings.url, settings.port);
    let url = url.as_str();
    let test_request = TestRequest {
        test_uuid: String::from("3"),
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
    let result: Vec<RunAndInteractResponse> = serde_json::from_str(&response.text().await.unwrap()).unwrap();
    assert_eq!(result.len(), 100);
    for i in result {
        match i {
            RunAndInteractResponse::Ok(_, result) => {
                assert_eq!(result.stdout, "AC with 101 steps\n".as_bytes().to_vec());
            },
            i => panic!("{:?}", i),
        }
    }
    Ok(())
}