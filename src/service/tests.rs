use actix_web::{post, web, HttpResponse};
use emjudge_judgecore::{program::RawCode, quantity::{MemorySize, ProcessResource}, result::{AnsAndEvalResult, OnlyRunResult, RunAndEvalResult, RunAndInteractResult}, test::{AnsAndEval, OnlyRun, RunAndEval, RunAndInteract}};
use serde::{Deserialize, Serialize};

use crate::{db::TestDBRequestResult, settings::Settings};
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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OnlyRunResponse {
    InternalError(String),
    Ok(ProcessResource),
    CompileError(String),
    RuntimeError(ProcessResource),
    TimeLimitExceeded(ProcessResource),
    MemoryLimitExceeded(ProcessResource),
    MaximumCodeLimitExceeded(MemorySize),
    NoSuchLanguage,
    TestDataNotFound,
    OutputLimitExceeded(ProcessResource),
}

impl From<OnlyRunResult> for OnlyRunResponse {
    fn from(result: OnlyRunResult) -> Self {
        match result {
            OnlyRunResult::PermissionDenied => OnlyRunResponse::InternalError(String::from("Permission denied")),
            OnlyRunResult::InternalError(error) => OnlyRunResponse::InternalError(error),
            OnlyRunResult::SettingError => OnlyRunResponse::InternalError(String::from("Setting error")),
            OnlyRunResult::Ok(resource) => OnlyRunResponse::Ok(resource),
            OnlyRunResult::CompileError(error) => OnlyRunResponse::CompileError(error),
            OnlyRunResult::RuntimeError(resource) => OnlyRunResponse::RuntimeError(resource),
            OnlyRunResult::TimeLimitExceeded(resource) => OnlyRunResponse::TimeLimitExceeded(resource),
            OnlyRunResult::MemoryLimitExceeded(resource) => OnlyRunResponse::MemoryLimitExceeded(resource),
            OnlyRunResult::OutputLimitExceeded(resource) => OnlyRunResponse::OutputLimitExceeded(resource),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RunAndEvalResponse {
    InternalError(String),
    Ok(ProcessResource, ProcessResource),
    CompileError(String),
    RuntimeError(ProcessResource, ProcessResource),
    TimeLimitExceeded(ProcessResource, ProcessResource),
    MemoryLimitExceeded(ProcessResource, ProcessResource),
    OutputLimitExceeded(ProcessResource, ProcessResource),
    EvalCompileError(String),
    EvalRuntimeError(ProcessResource, ProcessResource),
    EvalTimeLimitExceeded(ProcessResource, ProcessResource),
    EvalMemoryLimitExceeded(ProcessResource, ProcessResource),
    EvalOutputLimitExceeded(ProcessResource, ProcessResource),
    MaximumCodeLimitExceeded(MemorySize),
    NoSuchLanguage,
    TestDataNotFound,

}

impl From<RunAndEvalResult> for RunAndEvalResponse {
    fn from(result: RunAndEvalResult) -> Self {
        match result {
            RunAndEvalResult::PermissionDenied => RunAndEvalResponse::InternalError(String::from("Permission denied")),
            RunAndEvalResult::InternalError(error) => RunAndEvalResponse::InternalError(error),
            RunAndEvalResult::SettingError => RunAndEvalResponse::InternalError(String::from("Setting error")),
            RunAndEvalResult::Ok(resource, eval_resource) => RunAndEvalResponse::Ok(resource, eval_resource),
            RunAndEvalResult::CompileError(error) => RunAndEvalResponse::CompileError(error),
            RunAndEvalResult::RuntimeError(resource, eval_resource) => RunAndEvalResponse::RuntimeError(resource, eval_resource),
            RunAndEvalResult::TimeLimitExceeded(resource, eval_resource) => RunAndEvalResponse::TimeLimitExceeded(resource, eval_resource),
            RunAndEvalResult::MemoryLimitExceeded(resource, eval_resource) => RunAndEvalResponse::MemoryLimitExceeded(resource, eval_resource),
            RunAndEvalResult::EvalCompileError(error) => RunAndEvalResponse::EvalCompileError(error),
            RunAndEvalResult::EvalRuntimeError(resource, eval_resource) => RunAndEvalResponse::EvalRuntimeError(resource, eval_resource),
            RunAndEvalResult::EvalTimeLimitExceeded(resource, eval_resource) => RunAndEvalResponse::EvalTimeLimitExceeded(resource, eval_resource),
            RunAndEvalResult::EvalMemoryLimitExceeded(resource, eval_resource) => RunAndEvalResponse::EvalMemoryLimitExceeded(resource, eval_resource),
            RunAndEvalResult::OutputLimitExceeded(resource, eval_resource) => RunAndEvalResponse::OutputLimitExceeded(resource, eval_resource),
            RunAndEvalResult::EvalOutputLimitExceeded(resource, eval_resource) => RunAndEvalResponse::EvalOutputLimitExceeded(resource, eval_resource),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AnsAndEvalResponse {
    InternalError(String),
    Ok(ProcessResource),
    EvalCompileError(String),
    EvalRuntimeError(ProcessResource),
    EvalTimeLimitExceeded(ProcessResource),
    EvalMemoryLimitExceeded(ProcessResource),
    EvalOutputLimitExceeded(ProcessResource),
    TestDataNotFound,
}

impl From<AnsAndEvalResult> for AnsAndEvalResponse {
    fn from(result: AnsAndEvalResult) -> Self {
        match result {
            AnsAndEvalResult::PermissionDenied => AnsAndEvalResponse::InternalError(String::from("Permission denied")),
            AnsAndEvalResult::InternalError(error) => AnsAndEvalResponse::InternalError(error),
            AnsAndEvalResult::SettingError => AnsAndEvalResponse::InternalError(String::from("Setting error")),
            AnsAndEvalResult::Ok(resource) => AnsAndEvalResponse::Ok(resource),
            AnsAndEvalResult::EvalCompileError(error) => AnsAndEvalResponse::EvalCompileError(error),
            AnsAndEvalResult::EvalRuntimeError(resource) => AnsAndEvalResponse::EvalRuntimeError(resource),
            AnsAndEvalResult::EvalTimeLimitExceeded(resource) => AnsAndEvalResponse::EvalTimeLimitExceeded(resource),
            AnsAndEvalResult::EvalMemoryLimitExceeded(resource) => AnsAndEvalResponse::EvalMemoryLimitExceeded(resource),
            AnsAndEvalResult::EvalOutputLimitExceeded(resource) => AnsAndEvalResponse::EvalOutputLimitExceeded(resource),
        }
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum RunAndInteractResponse {
    InternalError(String),
    Ok(ProcessResource, ProcessResource),
    CompileError(String),
    RuntimeError(ProcessResource, ProcessResource),
    TimeLimitExceeded(ProcessResource, ProcessResource),
    MemoryLimitExceeded(ProcessResource, ProcessResource),
    OutputLimitExceeded(ProcessResource, ProcessResource),
    InteractorCompileError(String),
    InteractorRuntimeError(ProcessResource, ProcessResource),
    InteractorTimeLimitExceeded(ProcessResource, ProcessResource),
    InteractorMemoryLimitExceeded(ProcessResource, ProcessResource),
    InteractorOutputLimitExceeded(ProcessResource, ProcessResource),
    MaximumCodeLimitExceeded(MemorySize),
    NoSuchLanguage,
    TestDataNotFound,
}

impl From<RunAndInteractResult> for RunAndInteractResponse {
    fn from(result: RunAndInteractResult) -> Self {
        match result {
            RunAndInteractResult::PermissionDenied => RunAndInteractResponse::InternalError(String::from("Permission denied")),
            RunAndInteractResult::InternalError(error) => RunAndInteractResponse::InternalError(error),
            RunAndInteractResult::SettingError => RunAndInteractResponse::InternalError(String::from("Setting error")),
            RunAndInteractResult::Ok(resource, interactor_resource) => RunAndInteractResponse::Ok(resource, interactor_resource),
            RunAndInteractResult::CompileError(error) => RunAndInteractResponse::CompileError(error),
            RunAndInteractResult::RuntimeError(resource, interactor_resource) => RunAndInteractResponse::RuntimeError(resource, interactor_resource),
            RunAndInteractResult::TimeLimitExceeded(resource, interactor_resource) => RunAndInteractResponse::TimeLimitExceeded(resource, interactor_resource),
            RunAndInteractResult::MemoryLimitExceeded(resource, interactor_resource) => RunAndInteractResponse::MemoryLimitExceeded(resource, interactor_resource),
            RunAndInteractResult::InteractorCompileError(error) => RunAndInteractResponse::InteractorCompileError(error),
            RunAndInteractResult::InteractorRuntimeError(resource, interactor_resource) => RunAndInteractResponse::InteractorRuntimeError(resource, interactor_resource),
            RunAndInteractResult::InteractorTimeLimitExceeded(resource, interactor_resource) => RunAndInteractResponse::InteractorTimeLimitExceeded(resource, interactor_resource),
            RunAndInteractResult::InteractorMemoryLimitExceeded(resource, interactor_resource) => RunAndInteractResponse::InteractorMemoryLimitExceeded(resource, interactor_resource),
            RunAndInteractResult::InteractorOutputLimitExceeded(resource, interactor_resource) => RunAndInteractResponse::InteractorOutputLimitExceeded(resource, interactor_resource),
            RunAndInteractResult::OutputLimitExceeded(resource, interactor_resource) => RunAndInteractResponse::OutputLimitExceeded(resource, interactor_resource),
        }
    }
}

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