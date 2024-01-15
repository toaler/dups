use std::string::ParseError;
use std::time::SystemTime;
use chrono::{DateTime, FixedOffset, TimeZone, Utc};

pub fn system_time_to_string(sys_time: &SystemTime) -> String {
    let datetime: DateTime<Utc> = (*sys_time).into(); // Dereference the reference
    datetime.to_rfc3339()
}

pub fn str_to_system_time(input: &str) -> Result<std::time::SystemTime, chrono::ParseError> {
    // Parse the input string into a DateTime<FixedOffset>
    let datetime: DateTime<FixedOffset> = DateTime::parse_from_rfc3339(input)?;

    // Convert the DateTime<FixedOffset> to a SystemTime
    let system_time = datetime.with_timezone(&Utc).into();

    Ok(system_time)
}