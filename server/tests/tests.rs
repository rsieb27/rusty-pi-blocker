#[cfg(test)]
mod tests {
    use hamcrest2::prelude::*;
    use rust_dns_adblocker::blocklist::{add_domain, load_blocklist, remove_domain};
    use rust_dns_adblocker::dns_server::generate_nxdomain_response;
    use rust_dns_adblocker::logger::log_blocked_domain;
    use std::fs;

    #[test]
    fn test_generate_nxdomain_response_valid() {
        // Header: [Transaction ID, Flags, QDCOUNT, ANCOUNT, NSCOUNT, ARCOUNT]
        let mut request = Vec::new();

        // Transaction ID: 0x1234
        request.extend_from_slice(&[0x12, 0x34]);
        // Flags: standard query 0x0100
        request.extend_from_slice(&[0x01, 0x00]);
        // QDCOUNT: 1
        request.extend_from_slice(&[0x00, 0x01]);
        // ANCOUNT: 0
        request.extend_from_slice(&[0x00, 0x00]);
        // NSCOUNT: 0
        request.extend_from_slice(&[0x00, 0x00]);
        // ARCOUNT: 0
        request.extend_from_slice(&[0x00, 0x00]);

        // Question section: QNAME ("example.com")
        request.push(7); // length of "example"
        request.extend_from_slice(b"example");
        request.push(3); // length of "com"
        request.extend_from_slice(b"com");
        request.push(0); // null terminator for QNAME
        // QTYPE: A (0x0001)
        request.extend_from_slice(&[0x00, 0x01]);
        // QCLASS: IN (0x0001)
        request.extend_from_slice(&[0x00, 0x01]);

        let response = generate_nxdomain_response(&request);

        // Build the expected response manually:
        let mut expected = Vec::new();
        // Transaction ID (same as request)
        expected.extend_from_slice(&[0x12, 0x34]);
        // Flags: QR=1, RD=1, RA=1, RCODE=3  (0x81, 0x83)
        expected.extend_from_slice(&[0x81, 0x83]);
        // QDCOUNT: same as in the original request (bytes 4..6)
        expected.extend_from_slice(&request[4..6]);
        // ANCOUNT, NSCOUNT, ARCOUNT set to 0
        expected.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        // Copy the question section (from byte 12 until after QNAME, plus 4 bytes for QTYPE & QCLASS)
        let mut offset = 12;
        while offset < request.len() && request[offset] != 0 {
            expected.push(request[offset]);
            offset += 1;
        }
        // Append the 0 terminator (if present)
        if offset < request.len() {
            expected.push(0);
            offset += 1;
        } else {
            // If missing the terminator, the function would have returned an empty vector.
            expected.clear();
        }
        // Append QTYPE and QCLASS (4 bytes)
        if offset + 4 <= request.len() {
            expected.extend_from_slice(&request[offset..offset + 4]);
        }

        assert_that!(response, equal_to(expected));
    }

    #[test]
    fn test_generate_nxdomain_response_too_short() {
        let request = vec![0, 1, 2]; // shorter than 12 bytes
        let response = generate_nxdomain_response(&request);

        assert_that!(response.len(), equal_to(0));
    }


    fn setup_blocklist_file(contents: &str) {
        fs::write("blocked_domains.txt", contents).expect("Failed to write blocklist file");
    }

    fn cleanup_blocklist_file() {
        let _ = fs::remove_file("blocked_domains.txt");
    }

    #[test]
    fn test_load_blocklist_empty() {
        cleanup_blocklist_file();

        let blocklist = load_blocklist().expect("Failed to load blocklist");

        // Expect an empty vector when the file does not exist.
        assert_that!(blocklist, equal_to(Vec::<String>::new()));
    }

    #[test]
    fn test_load_blocklist_with_data() {
        cleanup_blocklist_file();

        let data = "example.com\nmalicious.com";

        setup_blocklist_file(data);

        let blocklist = load_blocklist().expect("Failed to load blocklist");
        let expected = vec!["example.com".to_string(), "malicious.com".to_string()];

        assert_that!(blocklist, equal_to(expected));

        cleanup_blocklist_file();
    }

    #[test]
    fn test_add_and_remove_domain() {
        cleanup_blocklist_file();

        // Add a domain and verify it appears in the blocklist.
        add_domain("test.com").expect("Failed to add domain");
        let blocklist = load_blocklist().expect("Failed to load blocklist");
        assert_that!(blocklist.clone(), equal_to(vec!["test.com".to_string()]));

        // Adding the same domain again should not duplicate it.
        add_domain("test.com").expect("Failed to add domain a second time");
        let count = blocklist.iter().filter(|d| *d == "test.com").count();
        assert_that!(count, equal_to(1));

        // Remove the domain and verify it is gone.
        remove_domain("test.com").expect("Failed to remove domain");
        let blocklist = load_blocklist().expect("Failed to load blocklist");
        assert_that!(blocklist.contains(&"test.com".to_string()), equal_to(false));
        cleanup_blocklist_file();
    }

    #[test]
    fn test_log_blocked_domain() {
        let log_file = "blocked_log.csv";
        // Remove any existing log file.
        let _ = fs::remove_file(log_file);

        // Log a blocked domain.
        log_blocked_domain("testdomain.com", "127.0.0.1").expect("Failed to log domain");

        // Read the log file and check that it contains the expected strings.
        let log_contents = fs::read_to_string(log_file).expect("Failed to read log file");
        assert_that!(log_contents.contains("testdomain.com"), equal_to(true));
        assert_that!(log_contents.contains("127.0.0.1"), equal_to(true));

        let _ = fs::remove_file(log_file);
    }

}