use std::time::Duration;

pub fn parse_to_duration(str: Option<&str>) -> Option<Duration> {
    let seconds: u64 = str?.parse().ok()?;
    Some(Duration::from_secs(seconds))
}
