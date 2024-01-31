use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct TestRequest {
    pub code: Vec<u8>,
    pub language: String,
    pub test_uuid: String,
}

#[derive(Deserialize, Serialize)]
pub struct AnsRequest {
    pub ans: Vec<u8>,
    pub test_uuid: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LanguageRequest {
    pub language: String,
}
