use chrono::Utc;
use std::fs::OpenOptions;
use std::io::Write;
use eyre::Result;

pub fn log_blocked_domain(domain: &str, source_ip: &str) -> Result<()> {
    let timestamp = Utc::now().to_rfc3339(); //get current time
    let log_entry = format!("{},{},{}\n", timestamp, domain, source_ip); //create the log line

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("blocked_log.csv")?;

    file.write_all(log_entry.as_bytes())?;
    Ok(())
}