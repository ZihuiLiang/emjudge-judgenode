use std::mem::size_of_val;

use emjudge_judgecore::quantity::{MemorySize, TimeSpan};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TestData {
    pub id: String,
    pub time_limit: TimeSpan,
    pub memory_limit: MemorySize,
    pub eval_or_interactor_time_limit: TimeSpan,
    pub eval_or_interactor_memory_limit: MemorySize,
    pub input: Vec<Vec<u8>>,
    pub output: Vec<Vec<u8>>,
    pub eval_or_interactor_code: Vec<u8>,
    pub eval_or_interactor_language: String,
}

impl TestData {
    pub fn calculate_db_storage_size(&self) -> usize {
        self.id.len() + size_of_val(&self.time_limit) + size_of_val(&self.memory_limit) + size_of_val(&self.eval_or_interactor_time_limit) + size_of_val(&self.eval_or_interactor_memory_limit) + self.input.len() * 8 + self.input.iter().map(|x| x.len()).sum::<usize>() + self.output.len() * 8 + self.output.iter().map(|x| x.len()).sum::<usize>() + self.eval_or_interactor_code.len() + self.eval_or_interactor_language.len()
    }

    pub fn test_size(&self) -> usize {
        self.input.iter().zip(self.output.iter()).map(|(x, y)| x.len() + y.len()).sum()
    }
}