use chrono::{DateTime, Utc};
use chrono::Duration;
// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let gigasecond = Duration::seconds(1000000000);
    let end: DateTime<Utc> = start.checked_add_signed(gigasecond).unwrap();
    return end;
}

