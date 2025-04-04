use axiom::behaviours::New;
use chrono::TimeZone;
use chrono::Utc;
use domain::time::Timestamp;
use use_cases::gateways::formatters::time::TimestampFormatter;

#[derive(New)]
pub struct Rfc2822TimestampFormatter;

impl TimestampFormatter for Rfc2822TimestampFormatter {
    fn format(&self, timestamp: Timestamp) -> String {
        let millis = timestamp.as_millis_since_epoch();
        let datetime = Utc
            .timestamp_millis_opt(millis)
            .single()
            .expect("Invalid timestamp");
        datetime.to_rfc2822()
    }
}
