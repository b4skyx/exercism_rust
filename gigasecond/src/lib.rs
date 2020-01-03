use chrono::Duration;
use chrono::{DateTime, Utc};
// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    const GIGASEC: i64 = 1_000_000_000;
    start + Duration::seconds(GIGASEC)
}
