use emjudge_judgecore::{quantity::{MemorySize, ProcessResource, TimeSpan}, result::{AnsAndEvalResult, OnlyRunResult, RunAndEvalResult, RunAndInteractResult}, settings::CompileAndExeSettings};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SubmitResponse {
    InternalError(String),
    Ok,
    MaximumPerTestDataLimitExceeded(MemorySize),
    MaximumTimeLimitExceeded(TimeSpan),
    MaximumMemoryLimitExceeded(MemorySize),
    MaximumEvalOrInteractorCodeLimitExceeded(MemorySize),
    NoSuchLanguageForEvalOrInteractor,
    InputLengthNotMatchOutputLength,
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


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiskInfo {
    pub total: MemorySize,
    pub free: MemorySize,
}

impl DiskInfo {
    pub fn new() -> Result<Self, String> {
        let disk = match sys_info::disk_info() {
            Ok(disk) => disk,
            Err(error) => return Err(format!("Cannot get disk info: {}", error)),
        };
        Ok(Self {
            total: MemorySize::from_bytes(disk.total as usize),
            free: MemorySize::from_bytes(disk.free as usize),
        })
    }

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CpuInfo {
    pub cpu_num: u32,
    pub cpu_speed_mhz: u32,
}

impl CpuInfo {
    pub fn new() -> Result<Self, String> {
        let cpu_num = match sys_info::cpu_num() {
            Ok(cpu_num) => cpu_num,
            Err(error) => return Err(format!("Cannot get cpu num: {}", error)),
        };
        let cpu_speed_mhz = match sys_info::cpu_speed() {
            Ok(cpu_speed_mhz) => cpu_speed_mhz as u32,
            Err(error) => return Err(format!("Cannot get cpu speed: {}", error)),
        };
        Ok(Self {
            cpu_num,
            cpu_speed_mhz,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MemInfo {
    pub total: MemorySize,
    pub free: MemorySize,
    pub avail: MemorySize,
    pub buffers: MemorySize,
    pub cached: MemorySize,
    pub swap_total: MemorySize,
    pub swap_free: MemorySize,
}

impl MemInfo {
    pub fn new() -> Result<Self, String> {
        let mem = match sys_info::mem_info() {
            Ok(mem) => mem,
            Err(error) => return Err(format!("Cannot get mem info: {}", error)),
        };
        Ok(Self {
            total: MemorySize::from_bytes(mem.total as usize),
            free: MemorySize::from_bytes(mem.free as usize),
            avail: MemorySize::from_bytes(mem.avail as usize),
            buffers: MemorySize::from_bytes(mem.buffers as usize),
            cached: MemorySize::from_bytes(mem.cached as usize),
            swap_total: MemorySize::from_bytes(mem.swap_total as usize),
            swap_free: MemorySize::from_bytes(mem.swap_free as usize),
        })
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SysInfo {
    pub os_type: String,
    pub os_release: String,
}

impl SysInfo {
    pub fn new() -> Result<Self, String> {
        let os_type = match sys_info::os_type() {
            Ok(os_type) => os_type,
            Err(error) => return Err(format!("Cannot get os type: {}", error)),
        };
        let os_release = match sys_info::os_release() {
            Ok(os_release) => os_release,
            Err(error) => return Err(format!("Cannot get os release: {}", error)),
        };
        Ok(Self {
            os_type,
            os_release,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllInfo {
    pub disk: DiskInfo,
    pub cpu: CpuInfo,
    pub mem: MemInfo,
    pub sys: SysInfo,
}

impl AllInfo {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            disk: DiskInfo::new()?,
            cpu: CpuInfo::new()?,
            mem: MemInfo::new()?,
            sys: SysInfo::new()?,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SysInfoResponse {
    Ok(SysInfo),
    Error(String),
}

impl From<Result<SysInfo, String>> for SysInfoResponse {
    fn from(result: Result<SysInfo, String>) -> Self {
        match result {
            Ok(result) => SysInfoResponse::Ok(result),
            Err(error) => SysInfoResponse::Error(error),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DiskInfoResponse {
    Ok(DiskInfo),
    Error(String),
}

impl From<Result<DiskInfo, String>> for DiskInfoResponse {
    fn from(result: Result<DiskInfo, String>) -> Self {
        match result {
            Ok(result) => DiskInfoResponse::Ok(result),
            Err(error) => DiskInfoResponse::Error(error),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CpuInfoResponse {
    Ok(CpuInfo),
    Error(String),
}

impl From<Result<CpuInfo, String>> for CpuInfoResponse {
    fn from(result: Result<CpuInfo, String>) -> Self {
        match result {
            Ok(result) => CpuInfoResponse::Ok(result),
            Err(error) => CpuInfoResponse::Error(error),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MemInfoResponse {
    Ok(MemInfo),
    Error(String),
}

impl From<Result<MemInfo, String>> for MemInfoResponse {
    fn from(result: Result<MemInfo, String>) -> Self {
        match result {
            Ok(result) => MemInfoResponse::Ok(result),
            Err(error) => MemInfoResponse::Error(error),
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AllInfoResponse {
    Ok(AllInfo),
    Error(String),
}

impl From<Result<AllInfo, String>> for AllInfoResponse {
    fn from(result: Result<AllInfo, String>) -> Self {
        match result {
            Ok(result) => AllInfoResponse::Ok(result),
            Err(error) => AllInfoResponse::Error(error),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguageInfo {
    pub language: String,
    pub version: String,
    pub compile_command: String,
    pub exe_command: String,
}

impl LanguageInfo {
    pub fn new(language: &str, compile_and_exe_settings: &CompileAndExeSettings) -> Result<Self, String> {
        match compile_and_exe_settings.get_language(language) {
            Some(compile_and_exe_setting) => Ok(Self {
                language: language.to_string(),
                version: {
                    match compile_and_exe_settings.get_language_info(language) {
                        Ok(language_info) => language_info,
                        Err(result) => result,
                    }
                },
                compile_command: compile_and_exe_setting.compile_command.clone(),
                exe_command: compile_and_exe_setting.exe_command.clone(),
            }),
            None => Err(format!("No such language: {}", language)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LanguageInfoResponse {
    Ok(LanguageInfo),
    Error(String),
}

impl From<Result<LanguageInfo, String>> for LanguageInfoResponse {
    fn from(result: Result<LanguageInfo, String>) -> Self {
        match result {
            Ok(result) => LanguageInfoResponse::Ok(result),
            Err(error) => LanguageInfoResponse::Error(error),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LanguagesInfo {
    pub languages: Vec<LanguageInfo>,
}

impl LanguagesInfo {
    pub fn new(compile_and_exe_settings: &CompileAndExeSettings) -> Self {
        Self {
            languages: compile_and_exe_settings.languages.iter().map(|(language, _)| LanguageInfo::new(language, compile_and_exe_settings).unwrap()).collect::<Vec<LanguageInfo>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LanguagesInfoResponse {
    Ok(LanguagesInfo),
}

impl From<LanguagesInfo> for LanguagesInfoResponse {
    fn from(result: LanguagesInfo) -> Self {
        LanguagesInfoResponse::Ok(result)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TestPressureResponse {
    Ok(TimeSpan),
}
