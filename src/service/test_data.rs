use std::borrow::Borrow;

use actix_web::{post, web, HttpResponse};

use crate::{data::TestData, responses::SubmitResponse, settings::Settings};

#[post("/submit")]
pub async fn submit(settings: web::Data<Settings>, req: web::Json<TestData>) -> HttpResponse {
    if req.input.len() != req.output.len() {
        return HttpResponse::Ok().json(SubmitResponse::InputLengthNotMatchOutputLength);
    }

    if req.time_limit > settings.max_time_limit
        || req.eval_or_interactor_time_limit > settings.max_time_limit
    {
        return HttpResponse::Ok().json(SubmitResponse::MaximumTimeLimitExceeded(
            settings.max_time_limit,
        ));
    }

    if req.memory_limit > settings.max_memory_limit
        || req.eval_or_interactor_memory_limit > settings.max_memory_limit
    {
        return HttpResponse::Ok().json(SubmitResponse::MaximumMemoryLimitExceeded(
            settings.max_memory_limit,
        ));
    }

    if req.test_size() > settings.max_per_test_data_limit.as_bytes() {
        return HttpResponse::Ok().json(SubmitResponse::MaximumPerTestDataLimitExceeded(
            settings.max_per_test_data_limit,
        ));
    }

    if req.eval_or_interactor_code.len() > settings.max_code_limit.as_bytes() {
        return HttpResponse::Ok().json(SubmitResponse::MaximumEvalOrInteractorCodeLimitExceeded(
            settings.max_code_limit,
        ));
    }

    if settings
        .compile_and_exe
        .languages
        .get(&req.eval_or_interactor_language)
        .is_none()
    {
        return HttpResponse::Ok().json(SubmitResponse::NoSuchLanguageForEvalOrInteractor);
    }

    match settings.test_db.insert(&req.borrow()) {
        Ok(_) => HttpResponse::Ok().json(SubmitResponse::Ok),
        Err(error) => HttpResponse::Ok().json(SubmitResponse::InternalError(error)),
    }
}
