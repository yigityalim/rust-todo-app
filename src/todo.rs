use chrono::{TimeZone, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub done: bool,
    pub created_at: u64,
    pub completed_at: Option<u64>,
}

impl Todo {
    pub fn new(id: Uuid, text: String) -> Todo {
        Todo {
            id,
            text,
            done: false,
            created_at: get_current_timestamp(),
            completed_at: None,
        }
    }

    pub fn mark_done(&mut self) {
        self.done = true;
        self.completed_at = Some(get_current_timestamp());
    }
}

pub fn get_current_timestamp() -> u64 {
    Utc::now().timestamp() as u64
}

pub fn format_timestamp(timestamp: u64) -> String {
    let datetime = Utc.timestamp_opt(timestamp as i64, 0).unwrap();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}