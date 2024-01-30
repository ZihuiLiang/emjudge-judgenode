use emjudge_judgecore::quantity::{MemorySize, TimeSpan};
use rusqlite::named_params;

use crate::data::TestData;

pub enum TestDBRequestResult {
    Ok(TestData),
    NoSuchTest,
    InternalError(String),
}

#[derive(Clone, Debug)]
pub struct TestDB {
    pub path: String,
    pub max_db_limit: MemorySize,
}

impl TestDB {
    pub fn new(path: String, max_db_limit: MemorySize) -> Result<Self, String> {
        let db = match rusqlite::Connection::open(&path) {
            Ok(db) => db,
            Err(error) => return Err(error.to_string()),
        };
        match db.execute("CREATE TABLE IF NOT EXISTS test (
            id TEXT PRIMARY KEY,
            time_limit_ms INTEGER,
            memory_limit_bytes INTEGER,
            eval_or_interactor_time_limit_ms INTEGER,
            eval_or_interactor_memory_limit_bytes INTEGER,
            input BLOB,
            output BLOB,
            eval_or_interactor_code BLOB,
            eval_or_interactor_language TEXT,
            update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )", []) {
            Ok(_) => (),
            Err(error) => return Err(error.to_string()),
        };
        Ok(
        Self {
            path,
            max_db_limit,
        })
    }
    pub fn request(&self, test_uuid: &String) -> TestDBRequestResult {
        let db = match rusqlite::Connection::open(&self.path) {
            Ok(db) => db,
            Err(error) => return TestDBRequestResult::InternalError(error.to_string()),
        };
        match db.query_row("SELECT * FROM test WHERE id = ?1", &[test_uuid], |row| {
            Ok(TestData {
                id: row.get(0)?,
                time_limit: TimeSpan::from_milliseconds(row.get::<_, u64>(1)?),
                memory_limit: MemorySize::from_bytes(row.get::<_, usize>(2)?),
                eval_or_interactor_time_limit: TimeSpan::from_milliseconds(row.get(3)?),
                eval_or_interactor_memory_limit: MemorySize::from_bytes(row.get(4)?),
                input: extract(&row.get(5)?),
                output: extract(&row.get(6)?),
                eval_or_interactor_code: row.get(7)?,
                eval_or_interactor_language: row.get(8)?,
            })
        }) {
            Ok(test_data) => TestDBRequestResult::Ok(test_data),
            Err(rusqlite::Error::QueryReturnedNoRows) => TestDBRequestResult::NoSuchTest,
            Err(error) => TestDBRequestResult::InternalError(error.to_string()),
        }
    }

    pub fn insert(&self, test_data: &TestData) -> Result<(), String> {
        let db = match rusqlite::Connection::open(&self.path) {
            Ok(db) => db,
            Err(error) => return Err(error.to_string()),
        };
        let new_data_size = test_data.calculate_db_storage_size();
        let mut page_count: usize = db.query_row("PRAGMA page_count", [], |row| row.get(0)).unwrap();
        let mut page_size: usize = db.query_row("PRAGMA page_size", [], |row| row.get(0)).unwrap();
        let mut db_size: usize = page_count * page_size;
        while new_data_size + db_size > self.max_db_limit.as_bytes() {
            db.execute("DELETE FROM test WHERE update_time = (SELECT MIN(update_time) FROM test)", []).unwrap();
            page_count = db.query_row("PRAGMA page_count", [], |row| row.get(0)).unwrap();
            page_size = db.query_row("PRAGMA page_size", [], |row| row.get(0)).unwrap();
            db_size = page_count * page_size;
        }
        db.execute("INSERT INTO test (id, time_limit_ms, memory_limit_bytes, eval_or_interactor_time_limit_ms, eval_or_interactor_memory_limit_bytes, input, output, eval_or_interactor_code, eval_or_interactor_language) VALUES (:id, :time_limit_ms, :memory_limit_bytes, :eval_or_interactor_time_limit_ms, :eval_or_interactor_memory_limit_bytes, :input, :output, :eval_or_interactor_code, :eval_or_interactor_language) ON CONFLICT (id) DO UPDATE SET time_limit_ms = :time_limit_ms, memory_limit_bytes = :memory_limit_bytes, eval_or_interactor_time_limit_ms = :eval_or_interactor_time_limit_ms, eval_or_interactor_memory_limit_bytes = :eval_or_interactor_memory_limit_bytes, input = :input, output = :output, eval_or_interactor_code = :eval_or_interactor_code, eval_or_interactor_language = :eval_or_interactor_language",
            named_params! {
                ":id": test_data.id,
                ":time_limit_ms": test_data.time_limit.as_milliseconds(),
                ":memory_limit_bytes": test_data.memory_limit.as_bytes(),
                ":eval_or_interactor_time_limit_ms": test_data.eval_or_interactor_time_limit.as_milliseconds(),
                ":eval_or_interactor_memory_limit_bytes": test_data.eval_or_interactor_memory_limit.as_bytes(),
                ":input": pack(&test_data.input),
                ":output": pack(&test_data.output),
                ":eval_or_interactor_code": test_data.eval_or_interactor_code,
                ":eval_or_interactor_language": test_data.eval_or_interactor_language,
            }).unwrap();
        Ok(())
    }
}

fn extract(data: &Vec<u8>) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let len = u64::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3], data[i + 4], data[i + 5], data[i + 6], data[i + 7]]) as usize;
        i += 8;
        result.push(data[i..i + len].to_vec());
        i += len;
    }
    result
}

fn pack(datas: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut result = Vec::new();
    for data in datas {
        let len = data.len();
        result.extend_from_slice(&len.to_le_bytes());
        result.extend_from_slice(data);
    }
    result
}