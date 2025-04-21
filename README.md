# Rust-Based DNS Blocker
## Overview

The **Rusty Pi Blocker** is a DNS-based ad blocker written primarily in Rust, designed to run on a Raspberry Pi or any other Linux/Windows-based machine. It intercepts DNS queries made by client devices and blocks domains found in a customizable blocklist by responding with a fabricated "NXDOMAIN" DNS response. If a queried domain is not in the blocklist, the DNS request is forwarded to an upstream resolver (Google DNS by default).

The project includes:

- A custom-built DNS server using low-level UDP sockets
- File-based domain blocking and persistent log storage
- A command-line interface (CLI) for user interaction
- Logging of blocked domains and source IPs to `blocked_log.csv`
- Cross platform support for stopping the DNS server
- Optional Python based analysis of blocked domain/IP logs through FFI (Foreign Function Interface)

---

## Changes During the Semester

This project evolved significantly over the semester. It began as a basic UDP listener and gradually grew into a DNS ad blocker with both object-oriented design principles and FFI capabilities.

Key developments include:

- Implementation of OS specific logic to stop the DNS server on both Windows and Unix platforms
- Enhanced CLI functionality using `clap`, including commands for starting, stopping, listing, adding, removing, and analyzing domains
- Introduction of a Python script (`analysis.py`) integrated through Rust FFI to analyze logs of blocked traffic
- Addition of unit testing using `hamcrest2` and `serial_test` to ensure isolation and repeatability

---

## Lessons Learned

Throughout the development of this project, several technical and conceptual lessons were learned:

- **Concurrency and Shared State**: Managing mutable state across threads in Rust requires `Arc<Mutex<T>>`. This helped reinforce understanding of Rust’s ownership model and its safety guarantees.
- **Cross-Platform Design**: Writing code that works on both Windows and Unix requires careful use of conditional compilation via `#[cfg(...)]` and separate implementations for process control.
- **Error Handling**: Transitioning from panics to graceful error handling using `color-eyre` improved reliability and maintainability.
- **FFI Integration**: Interfacing Rust with Python using PyO3 was straightforward, enabling advanced log analysis without sacrificing performance or safety.
- **Test Hygiene**: Testing systems with persistent side effects (e.g., file I/O) necessitates careful use of cleanup functions and serialization to avoid cross-test interference.

---

## Future Work 

If given an additional month of development time, I'd consider adding the following enhancements and features:

1. IPv6 Compatibility: Add native IPv6 DNS query support / dual IPv4 and IPv6 resolution capability.
2. DNS Response Caching: Implement a local cache to reduce redundant upstream DNS requests and improve performance.
3. Advanced Log Analytics: Expand the Python log analysis to include statistical trends, charts, and export options.
4. Enhanced Blocklist Management: Support multiple blocklists, allowlists, and filter categories (e.g., ads, trackers, malware).
