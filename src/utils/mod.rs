use chrono::{ Datelike, Utc };
use rand::Rng;

pub mod http;
pub mod images;
pub mod mongodb;
pub mod types;

pub mod settings;
pub mod caching_info;

pub fn get_timestamp() -> i64 {
    let current_timestamp = Utc::now();
    current_timestamp.timestamp()
}
pub fn get_date_string() -> String {
    let current_timestamp = Utc::now();
    format!(
        "{}:{}:{}",
        current_timestamp.day(),
        current_timestamp.month(),
        current_timestamp.year()
    )
}
pub fn get_yesterday_date_string() -> String {
    let current_timestamp = Utc::now();
    format!(
        "{}:{}:{}",
        current_timestamp.day() - 1,
        current_timestamp.month(),
        current_timestamp.year()
    )
}
pub fn generate_id() -> i64 {
    let mut rng = rand::thread_rng();
    let current_timestamp = Utc::now().timestamp_millis() + rng.gen_range(0..999999);
    current_timestamp
}
