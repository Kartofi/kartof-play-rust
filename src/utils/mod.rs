use chrono::Utc;

pub mod http;
pub mod mongodb;
pub mod types;

pub fn get_timestamp() -> i64 {
    let current_timestamp = Utc::now();
    current_timestamp.timestamp()
}
