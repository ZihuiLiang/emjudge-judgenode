use std::borrow::Borrow;

use actix_web::{post, web, HttpResponse};
use emjudge_judgecore::quantity::{MemorySize, TimeSpan};
use serde::Serialize;

use crate::{db::TestData, settings::Settings};

#[derive(Serialize)]
pub enum SubmitResult {
    InternalError(String),
    Ok,
    MaximumPerTestDataLimitExceeded(MemorySize),
    MaximumTimeLimitExceeded(TimeSpan),
    MaximumMemoryLimitExceeded(MemorySize),
    MaximumEvalOrInteractorCodeLimitExceeded(MemorySize),
    NoSuchLanguageForEvalOrInteractor,
    InputLengthNotMatchOutputLength,
}


#[post("/submit")]
pub async fn submit(settings: web::Data<Settings>, req: web::Json<TestData>) -> HttpResponse {
    if req.input.len() != req.output.len() {
        return HttpResponse::Ok().json(SubmitResult::InputLengthNotMatchOutputLength);
    }

    if req.time_limit > settings.max_time_limit || req.eval_or_interactor_time_limit > settings.max_time_limit{
        return HttpResponse::Ok().json(SubmitResult::MaximumTimeLimitExceeded(settings.max_time_limit));
    }

    if req.memory_limit > settings.max_memory_limit || req.eval_or_interactor_memory_limit > settings.max_memory_limit{
        return HttpResponse::Ok().json(SubmitResult::MaximumMemoryLimitExceeded(settings.max_memory_limit));
    }

    if req.test_size() > settings.max_per_test_data_limit.as_bytes() {
        return HttpResponse::Ok().json(SubmitResult::MaximumPerTestDataLimitExceeded(settings.max_per_test_data_limit));
    }

    if req.eval_or_interactor_code.len() > settings.max_code_limit.as_bytes() {
        return HttpResponse::Ok().json(SubmitResult::MaximumEvalOrInteractorCodeLimitExceeded(settings.max_code_limit));
    }

    if settings.compile_and_exe.languages.get(&req.eval_or_interactor_language).is_none() {
        return HttpResponse::Ok().json(SubmitResult::NoSuchLanguageForEvalOrInteractor);
    }

    match settings.test_db.insert(&req.borrow()) {
        Ok(_) => HttpResponse::Ok().json(SubmitResult::Ok),
        Err(error) => HttpResponse::Ok().json(SubmitResult::InternalError(error)),
    }
}

