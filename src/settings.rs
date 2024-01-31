use std::sync::{Arc, RwLock};

use config::Config;
use emjudge_judgecore::{quantity::{MemorySize, TimeSpan}, settings::{create_a_tmp_user_return_uid, CompileAndExeSettings}};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use tokio::sync::Semaphore;

use crate::db::TestDB;

#[serde_as]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Settings {
    #[serde(default = "Settings::url_default")]
    pub url: String,
    #[serde(default = "Settings::port_default")]
    pub port: u16,
    #[serde(default = "Settings::test_threads_default")]
    pub test_threads: usize,
    #[serde(default = "Settings::actix_workers_default")]
    pub actix_workers: usize, 
    #[serde(default = "Settings::max_json_limit_default")]
    pub max_json_limit : MemorySize,
    #[serde(default = "Settings::max_time_limit_default")]
    pub max_time_limit: TimeSpan,
    #[serde(default = "Settings::max_memory_limit_default")]
    pub max_memory_limit: MemorySize,
    #[serde(default = "Settings::max_per_test_data_limit_default")]
    pub max_per_test_data_limit: MemorySize,
    #[serde(default = "Settings::max_code_limit_default")]
    pub max_code_limit: MemorySize,
    #[serde(default = "Settings::max_db_limit_default")]
    pub max_db_limit: MemorySize,
    #[serde(default = "Settings::output_limit_default")]
    pub output_limit: MemorySize,
    #[serde(default = "Settings::compile_and_exe_default")]
    pub compile_and_exe: CompileAndExeSettings,
    #[serde(default = "Settings::license_default")]
    pub license: String,
    #[serde(default = "Settings::tested_user_name_default")]
    pub tested_user_name: String,
    #[serde(default = "Settings::eval_or_interactor_user_name_default")]
    pub eval_or_interactor_user_name: String,
    #[serde(skip, default = "Settings::tested_uid_default")]
    pub tested_uid: u32,
    #[serde(skip, default = "Settings::eval_or_interactor_uid_default")]
    pub eval_or_interactor_uid: u32,
    #[serde(default = "Settings::test_data_db_path_default")]
    pub test_data_db_path: String,
    #[serde(skip, default = "Settings::acquire_for_test_thread_default")]
    pub acquire_for_test_thread: Arc<Semaphore>,
    #[serde(skip, default = "Settings::test_db_default")]
    pub test_db: TestDB,
    #[serde(skip, default = "Settings::test_pressure_default")]
    test_pressure: Arc<RwLock<TimeSpan>>
}

impl Settings {
    fn url_default() -> String {
        String::from("127.0.0.1")
    }
    fn port_default() -> u16 {
        8080
    }
    fn test_threads_default() -> usize {
        1
    }
    fn actix_workers_default() -> usize {
        1
    }
    fn max_json_limit_default() -> MemorySize {
        MemorySize::from_megabytes(10)
    }
    fn max_time_limit_default() -> TimeSpan {
        TimeSpan::from_seconds(1)
    }
    fn max_memory_limit_default() -> MemorySize {
        MemorySize::from_megabytes(256)
    }
    fn max_per_test_data_limit_default() -> MemorySize {
        MemorySize::from_megabytes(10)
    }
    fn max_code_limit_default() -> MemorySize {
        MemorySize::from_kilobytes(16)
    }
    fn max_db_limit_default() -> MemorySize {
        MemorySize::from_gigabytes(1)
    }
    fn output_limit_default() -> MemorySize {
        MemorySize::from_megabytes(10)
    }
    fn compile_and_exe_default() -> CompileAndExeSettings {
        CompileAndExeSettings::default()
    }
    fn license_default() -> String {
        String::from("license")
    }
    pub fn acquire_for_test_thread_default() -> Arc<Semaphore> {
        Arc::new(Semaphore::new(1))
    }
    fn tested_user_name_default() -> String {
        String::from("tested")
    }
    fn eval_or_interactor_user_name_default() -> String {
        String::from("eval_or_interactor")
    }
    fn tested_uid_default() -> u32 {
        0
    }
    fn eval_or_interactor_uid_default() -> u32 {
        0
    }
    fn test_data_db_path_default() -> String {
        String::from("db.sqlite3")
    }
    fn test_db_default() -> TestDB {
        TestDB {path: Self::test_data_db_path_default(), max_db_limit: Self::max_db_limit_default()}
    }
    fn test_pressure_default() -> Arc<RwLock<TimeSpan>> {
        Arc::new(RwLock::new(TimeSpan::from_seconds(0)))
    }
}

impl Settings {
    pub fn load_from_file(file_path: &str, format: config::FileFormat) -> Result<Self, String> {
        match Config::builder()
            .add_source(config::File::with_name(file_path).format(format))
            .build()
        {
            Ok(config) => match config.try_deserialize::<Self>() {
                Ok(mut result) => {
                    result.acquire_for_test_thread = Arc::new(Semaphore::new(result.test_threads));
                    result.test_db = TestDB::new(result.test_data_db_path.clone(), result.max_db_limit).unwrap();
                    result.tested_uid = create_a_tmp_user_return_uid(&result.tested_user_name).unwrap();
                    result.eval_or_interactor_uid = create_a_tmp_user_return_uid(&result.eval_or_interactor_user_name).unwrap();
                    Ok(result)
                },
                Err(result) => Err(result.to_string()),
            },
            Err(result) => Err(result.to_string()),
        }
    }

    pub fn add_test_pressure(&self, time: TimeSpan) {
        let mut test_pressure = self.test_pressure.write().unwrap();
        *test_pressure = *test_pressure + time;
    }

    pub fn sub_test_pressure(&self, time: TimeSpan) {
        let mut test_pressure = self.test_pressure.write().unwrap();
        *test_pressure = *test_pressure - time;
    }

    pub fn get_test_pressure(&self) -> TimeSpan {
        let test_pressure = self.test_pressure.read().unwrap();
        *test_pressure
    }
}
