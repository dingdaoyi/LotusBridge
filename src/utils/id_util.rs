use chrono::Utc;
use rand::random;

pub fn generate_unique_id() -> i64 {
    let timestamp = Utc::now().timestamp_millis();
    let random_number: i64 = random();
    timestamp + random_number
}
