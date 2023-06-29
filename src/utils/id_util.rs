use chrono::Utc;
use rand::Rng;

pub fn generate_unique_id() -> i64 {
    let timestamp = Utc::now().timestamp_millis();
    let random_number :i64= rand::thread_rng().gen();
    timestamp + random_number
}