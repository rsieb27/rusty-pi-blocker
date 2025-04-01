use std::fs;

const BLOCKLIST_FILE: &str = "blocked_domains.txt";

pub fn load_blocklist() -> Vec<String> {
    fs::read_to_string(BLOCKLIST_FILE)
        .unwrap_or_default()
        .lines()
        .map(String::from)
        .collect()

}

pub fn add_domain(domain: &str) {
    let mut list = load_blocklist();
    if !list.contains(&domain.to_string()) {
        list.push(domain.to_string());
        fs::write(BLOCKLIST_FILE, list.join("\n")).expect("Failed to write blocklist");
        println!("Domain '{}' added tp blocklst", domain);
    }
}

pub fn remove_domain(domain: &str) {
    let mut list = load_blocklist();
    list.retain(|d| d != domain);
    fs::write(BLOCKLIST_FILE, list.join("\n")).expect("Failed top write blocklist");
    println!("DOmain '{}' removed from blocklist", domain);
}

pub fn list_blocked_domains() {
    let list = load_blocklist();
    if list.is_empty() {
        println!("No domains currently blocked");
    } else {
        println!("Blocked Domains:");
        for domain in list {
            println!("-{}", domain);
        }
    }
}