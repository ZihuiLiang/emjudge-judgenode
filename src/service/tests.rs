use actix_web::{post, web, HttpResponse};
use emjudge_judgecore::{program::RawCode, test::{AnsAndEval, OnlyRun, RunAndEval, RunAndInteract}};

use crate::{db::TestDBRequestResult, requests::{AnsRequest, TestRequest}, responses::{AnsAndEvalResponse, OnlyRunResponse, RunAndEvalResponse, RunAndInteractResponse}, settings::Settings};




#[post("/only_run")]
pub async fn only_run(settings: web::Data<Settings>, req: web::Json<TestRequest>) -> HttpResponse {
    if req.code.len() > settings.max_code_limit.as_bytes() {
        return HttpResponse::Ok().json(OnlyRunResponse::MaximumCodeLimitExceeded(settings.max_code_limit));
    }
    let compile_and_exe_setting =  match settings.compile_and_exe.get_language(&req.language) {
        Some(compile_and_exe_setting) => compile_and_exe_setting,
        None => return HttpResponse::Ok().json(OnlyRunResponse::NoSuchLanguage),
    };
    let test_data = match settings.test_db.request(&req.test_uuid) {
        TestDBRequestResult::Ok(test_data) => test_data,
        TestDBRequestResult::NoSuchTest => return HttpResponse::Ok().json(OnlyRunResponse::TestDataNotFound),
        TestDBRequestResult::InternalError(error) => return HttpResponse::Ok().json(OnlyRunResponse::InternalError(error)),
    };
    let permission = settings.acquire_for_test_thread.clone().acquire_owned().await;
    let result = OnlyRun::multiple(&RawCode::new(&req.code, compile_and_exe_setting), test_data.time_limit, test_data.memory_limit, settings.tested_uid, &test_data.input, settings.output_limit).await;
    drop(permission);
    HttpResponse::Ok().json(result.iter().map(|x| OnlyRunResponse::from(x.clone())).collect::<Vec<OnlyRunResponse>>().to_vec())
}

#[post("/run_and_eval")]
pub async fn run_and_eval(settings: web::Data<Settings>, req: web::Json<TestRequest>) -> HttpResponse {
    if req.code.len() > settings.max_code_limit.as_bytes() {
        return HttpResponse::Ok().json(RunAndEvalResponse::MaximumCodeLimitExceeded(settings.max_code_limit));
    }
    let compile_and_exe_setting =  match settings.compile_and_exe.get_language(&req.language) {
        Some(compile_and_exe_setting) => compile_and_exe_setting,
        None => return HttpResponse::Ok().json(RunAndEvalResponse::NoSuchLanguage),
    };
    let test_data = match settings.test_db.request(&req.test_uuid) {
        TestDBRequestResult::Ok(test_data) => test_data,
        TestDBRequestResult::NoSuchTest => return HttpResponse::Ok().json(RunAndEvalResponse::TestDataNotFound),
        TestDBRequestResult::InternalError(error) => return HttpResponse::Ok().json(RunAndEvalResponse::InternalError(error)),
    };
    let eval_compile_and_exe_setting =  settings.compile_and_exe.get_language(&test_data.eval_or_interactor_language).unwrap();
    let permission = settings.acquire_for_test_thread.clone().acquire_owned().await;
    let result = RunAndEval::multiple(&RawCode::new(&req.code, compile_and_exe_setting), test_data.time_limit, test_data.memory_limit, settings.tested_uid, &RawCode::new(&test_data.eval_or_interactor_code, eval_compile_and_exe_setting), test_data.eval_or_interactor_time_limit, test_data.eval_or_interactor_memory_limit, settings.eval_or_interactor_uid, &test_data.input, &test_data.output, settings.output_limit).await;
    drop(permission);
    HttpResponse::Ok().json(result.iter().map(|x| RunAndEvalResponse::from(x.clone())).collect::<Vec<RunAndEvalResponse>>().to_vec())
}


#[post("/ans_and_eval")]
pub async fn ans_and_eval(settings: web::Data<Settings>, req: web::Json<AnsRequest>) -> HttpResponse {
    let test_data = match settings.test_db.request(&req.test_uuid) {
        TestDBRequestResult::Ok(test_data) => test_data,
        TestDBRequestResult::NoSuchTest => return HttpResponse::Ok().json(AnsAndEvalResponse::TestDataNotFound),
        TestDBRequestResult::InternalError(error) => return HttpResponse::Ok().json(AnsAndEvalResponse::InternalError(error)),
    };
    let eval_compile_and_exe_setting =  settings.compile_and_exe.get_language(&test_data.eval_or_interactor_language).unwrap();
    let permission = settings.acquire_for_test_thread.clone().acquire_owned().await;
    let result = AnsAndEval::multiple(&RawCode::new(&test_data.eval_or_interactor_code, eval_compile_and_exe_setting), test_data.eval_or_interactor_time_limit, test_data.eval_or_interactor_memory_limit, settings.eval_or_interactor_uid, &test_data.input,&test_data.output, settings.output_limit).await;
    drop(permission);
    HttpResponse::Ok().json(result.iter().map(|x| AnsAndEvalResponse::from(x.clone())).collect::<Vec<AnsAndEvalResponse>>().to_vec())
}


#[post("/run_and_interact")]
pub async fn run_and_interact(settings: web::Data<Settings>, req: web::Json<TestRequest>) -> HttpResponse {
    if req.code.len() > settings.max_code_limit.as_bytes() {
        return HttpResponse::Ok().json(RunAndInteractResponse::MaximumCodeLimitExceeded(settings.max_code_limit));
    }
    let compile_and_exe_setting =  match settings.compile_and_exe.get_language(&req.language) {
        Some(compile_and_exe_setting) => compile_and_exe_setting,
        None => return HttpResponse::Ok().json(RunAndInteractResponse::NoSuchLanguage),
    };
    let test_data = match settings.test_db.request(&req.test_uuid) {
        TestDBRequestResult::Ok(test_data) => test_data,
        TestDBRequestResult::NoSuchTest => return HttpResponse::Ok().json(RunAndInteractResponse::TestDataNotFound),
        TestDBRequestResult::InternalError(error) => return HttpResponse::Ok().json(RunAndInteractResponse::InternalError(error)),
    };
    let interactor_compile_and_exe_setting =  settings.compile_and_exe.get_language(&test_data.eval_or_interactor_language).unwrap();
    let permission = settings.acquire_for_test_thread.clone().acquire_owned().await;
    let result = RunAndInteract::multiple(&RawCode::new(&req.code, compile_and_exe_setting), test_data.time_limit, test_data.memory_limit, settings.tested_uid, &RawCode::new(&test_data.eval_or_interactor_code, interactor_compile_and_exe_setting), test_data.eval_or_interactor_time_limit, test_data.eval_or_interactor_memory_limit, settings.eval_or_interactor_uid, &test_data.input, settings.output_limit).await;
    drop(permission);
    HttpResponse::Ok().json(result.iter().map(|x| RunAndInteractResponse::from(x.clone())).collect::<Vec<RunAndInteractResponse>>().to_vec())
}