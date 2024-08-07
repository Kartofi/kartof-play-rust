use chrono::Utc;
use rand::Rng;

pub mod http;
pub mod mongodb;
pub mod types;

pub fn get_timestamp() -> i64 {
    let current_timestamp = Utc::now();
    current_timestamp.timestamp()
}
pub fn generate_id() -> i64 {
    let mut rng = rand::thread_rng();
    let current_timestamp = Utc::now().timestamp_millis() + rng.gen_range(0..999999);
    current_timestamp
}
