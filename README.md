# rusty-pi-blocker
# Rust DNS Ad Blocker

A simple DNS-based ad blocker written in Rust.  
Logs blocked domains to `blocked_log.csv` for Python-based analysis.

## Features
- DNS interception (port 53)
- CLI to add/remove/list blocked domains
- Python script to analyze block logs

## Folder Structure
- `server/`: Rust code and tests
- `analysis/`: Python script for log analysis
- `blocked_domains.txt`: initial blocklist
