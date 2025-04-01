use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_blocked_domain(domain: &str, source_ip: &str) {
    let timestamp = Utc::now().to_rfc3339();
    let log_entry = format!("{},{},{}\n", timestamp, domain, source_ip);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("blocked_log.csv")
        .expect("failed to open log file");

    file.write_all(log_entry.as_bytes())
        .expect("failed to write to log file");
}