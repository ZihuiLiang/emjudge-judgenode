use emjudge_judgecore::quantity::{MemorySize, TimeSpan};
use emjudge_judgenode::{data::TestData, settings::Settings};
use reqwest::{header::{HeaderMap, HeaderValue}, Error};

#[tokio::test]
async fn test_data_submit() -> Result<(), Error> {
    let settings = Settings::load_from_file("config/settings.toml", config::FileFormat::Toml).unwrap();
    let license = settings.license;
    let mut headers = HeaderMap::new();
    let url = format!("http://{}:{}/test_data/submit", settings.url, settings.port);
    let url = url.as_str();
    headers.insert("Authorization", HeaderValue::from_str(format!("Bearer {}", license).as_str()).unwrap());
    let test_data = TestData {
        id: String::from("4"),
        input: vec![vec![];1],
        output: vec![vec![];2],
        time_limit: settings.max_time_limit,
        memory_limit: settings.max_memory_limit,
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
        eval_or_interactor_code: vec![],
        eval_or_interactor_language: String::from("C++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .json(&test_data)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);    
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::InputLengthNotMatchOutputLength).unwrap());
    let test_data = TestData {
        id: String::from("4"),
        input: vec![],
        output: vec![],
        time_limit: TimeSpan::from_seconds(1) + settings.max_time_limit,
        memory_limit: settings.max_memory_limit,
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
        eval_or_interactor_code: vec![],
        eval_or_interactor_language: String::from("C++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .json(&test_data)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);    
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::MaximumTimeLimitExceeded(settings.max_time_limit)).unwrap());
    let test_data = TestData {
        id: String::from("4"),
        input: vec![],
        output: vec![],
        time_limit: settings.max_time_limit,
        memory_limit: settings.max_memory_limit,
        eval_or_interactor_time_limit: TimeSpan::from_seconds(1) + settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
        eval_or_interactor_code: vec![],
        eval_or_interactor_language: String::from("C++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .json(&test_data)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);    
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::MaximumTimeLimitExceeded(settings.max_time_limit)).unwrap());
    let test_data = TestData {
        id: String::from("4"),
        input: vec![],
        output: vec![],
        time_limit: settings.max_time_limit,
        memory_limit: MemorySize::from_bytes(1) + settings.max_memory_limit,
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
        eval_or_interactor_code: vec![],
        eval_or_interactor_language: String::from("C++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .headers(headers.clone())
        .json(&test_data)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::MaximumMemoryLimitExceeded(settings.max_memory_limit)).unwrap());
    let test_data = TestData {
        id: String::from("4"),
        input: vec![],
        output: vec![],
        time_limit: settings.max_time_limit,
        memory_limit: settings.max_memory_limit,
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: MemorySize::from_bytes(1) + settings.max_memory_limit,
        eval_or_interactor_code: vec![],
        eval_or_interactor_language: String::from("C++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .headers(headers.clone())
        .json(&test_data)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::MaximumMemoryLimitExceeded(settings.max_memory_limit)).unwrap());
    let test_data = TestData {
        id: String::from("4"),
        input: vec![vec![0]],
        output: vec![vec![0;settings.max_per_test_data_limit.as_bytes() as usize]],
        time_limit: settings.max_time_limit,
        memory_limit: settings.max_memory_limit,
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
        eval_or_interactor_code: vec![],
        eval_or_interactor_language: String::from("C++"),
    };

    let response = reqwest::Client::new()
        .post(url)
        .headers(headers.clone())
        .json(&test_data)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::MaximumPerTestDataLimitExceeded(settings.max_per_test_data_limit)).unwrap());
    let test_data = TestData {
        id: String::from("4"),
        input: vec![vec![0]; settings.max_per_test_data_limit.as_bytes() as usize / 2 + 1],
        output: vec![vec![0]; settings.max_per_test_data_limit.as_bytes() as usize / 2 + 1],
        time_limit: settings.max_time_limit,
        memory_limit: settings.max_memory_limit,
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
        eval_or_interactor_code: vec![0;settings.max_code_limit.as_bytes() as usize],
        eval_or_interactor_language: String::from("C++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .headers(headers.clone())
        .json(&test_data)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::MaximumPerTestDataLimitExceeded(settings.max_per_test_data_limit)).unwrap());
    let test_data = TestData {
        id: String::from("4"),
        input: vec![vec![0;settings.max_per_test_data_limit.as_bytes() as usize]],
        output: vec![vec![]],
        time_limit: settings.max_time_limit,
        memory_limit: settings.max_memory_limit,
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
        eval_or_interactor_code: vec![0;settings.max_code_limit.as_bytes() as usize + 1],
        eval_or_interactor_language: String::from("C++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .headers(headers.clone())
        .json(&test_data)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::MaximumEvalOrInteractorCodeLimitExceeded(settings.max_code_limit)).unwrap());
    let test_data = TestData {
        id: String::from("4"),
        input: vec![vec![0;settings.max_per_test_data_limit.as_bytes() as usize]],
        output: vec![vec![]],
        time_limit: settings.max_time_limit,
        memory_limit: settings.max_memory_limit,
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
        eval_or_interactor_code: vec![0;settings.max_code_limit.as_bytes() as usize],
        eval_or_interactor_language: String::from("D++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .headers(headers.clone())
        .json(&test_data)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::NoSuchLanguageForEvalOrInteractor).unwrap());
    let test_data = TestData {
        id: String::from("4"),
        input: vec![vec![0;settings.max_per_test_data_limit.as_bytes() as usize]],
        output: vec![vec![]],
        time_limit: settings.max_time_limit,
        memory_limit: settings.max_memory_limit,
        eval_or_interactor_time_limit: settings.max_time_limit,
        eval_or_interactor_memory_limit: settings.max_memory_limit,
        eval_or_interactor_code: vec![0;settings.max_code_limit.as_bytes() as usize],
        eval_or_interactor_language: String::from("C++"),
    };
    let response = reqwest::Client::new()
        .post(url)
        .headers(headers.clone())
        .json(&test_data)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.text().await.unwrap(), serde_json::to_string(&emjudge_judgenode::responses::SubmitResponse::Ok).unwrap());
    Ok(())
}   