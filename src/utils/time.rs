use chrono::{DateTime, Utc};
use std::time::Instant;

/// Formats a provided duration string into a human readable date string.
pub fn humanize_duration(duration: DateTime<Utc>) -> String {
    let now = Utc::now();
    let duration = now.signed_duration_since(duration);

    let days = duration.num_days();
    let years = days / 365;
    let weeks = (days % 365) / 7;
    let remaining_days = days % 7;

    let mut parts = Vec::new();

    if years > 0 {
        parts.push(format!("{years} year{}", if years > 1 { "s" } else { "" }));
    }

    if weeks > 0 {
        parts.push(format!("{weeks} week{}", if weeks > 1 { "s" } else { "" }));
    }

    if remaining_days > 0 || parts.is_empty() {
        parts.push(format!("{remaining_days} day{}", if remaining_days > 1 { "s" } else { "" }));
    }

    format!("{} ago", parts.join(" and "))
}

/// Format an `Instant` into a human-readable uptime string (e.g., "2 hours, 15 minutes, 8 seconds")
pub fn format_uptime(start_time: Instant) -> String {
    let elapsed = start_time.elapsed().as_secs();
    let days = elapsed / 86400;
    let hours = (elapsed % 86400) / 3600;
    let minutes = (elapsed % 3600) / 60;
    let seconds = elapsed % 60;

    let mut parts = Vec::new();

    if days > 0 {
        parts.push(format!("{days}d"));
    }

    if hours > 0 {
        parts.push(format!("{hours}h"));
    }

    if minutes > 0 {
        parts.push(format!("{minutes}m"));
    }

    parts.push(format!("{seconds}s"));
    parts.join(" ")
}
