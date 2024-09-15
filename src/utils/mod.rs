use chrono::{Datelike, Duration, TimeDelta, Utc};
use rand::Rng;

pub mod http;
pub mod images;
pub mod mongodb;
pub mod types;

pub mod caching_info;
pub mod settings;

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
pub fn get_previous_date_string(ago: &i64) -> String {
    let mut current_timestamp = Utc::now();
    current_timestamp -= Duration::from(TimeDelta::days(*ago));
    format!(
        "{}:{}:{}",
        current_timestamp.day(),
        current_timestamp.month(),
        current_timestamp.year()
    )
}
pub fn generate_id() -> i64 {
    let mut rng = rand::thread_rng();
    let current_timestamp = Utc::now().timestamp_millis() + rng.gen_range(0..999999);
    current_timestamp
}
