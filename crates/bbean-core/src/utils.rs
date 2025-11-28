use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn format_duration(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    }
}

pub fn truncate_id(id: &str, len: usize) -> &str {
    if id.len() <= len {
        id
    } else {
        &id[..len]
    }
}

pub fn validate_hex(s: &str) -> bool {
    s.len() % 2 == 0 && s.chars().all(|c| c.is_ascii_hexdigit())
}
