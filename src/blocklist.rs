use std::fs;
use std::path::Path;
use eyre::Result;

pub fn load_blocklist() -> Result<Vec<String>> {
    if !Path::new("blocked_domains.txt").exists() {
        return Ok(vec![]);
    }
    let contents = fs::read_to_string("blocked_domains.txt")?;
    Ok(contents.lines().map(String::from).collect())
}

pub fn add_domain(domain: &str) -> Result<()> {
    let mut list = load_blocklist()?;
    if list.contains(&domain.to_string()) {
        println!("Domain '{}' is already in blocklst", domain);
        return Ok(());
    }
    list.push(domain.to_string());
    fs::write("blocked_domains.txt", list.join("\n"))?;
    println!("Domain '{}' added to blocklst", domain);
    Ok(())
}

pub fn remove_domain(domain: &str) -> Result<()> {
    let mut list = load_blocklist()?;
    let initial_len = list.len();
    list.retain(|d| d != domain);
    if list.len() == initial_len {
        println!("Domain '{}' was not in blocklist", domain);
    } else {
        fs::write("blocked_domains.txt", list.join("\n"))?;
        println!("Domain '{}' removed from blocklist", domain);
    }
    Ok(())
}

pub fn list_blocked_domains(_path: &str) -> Result<()> {
    let list = load_blocklist()?;
    if list.is_empty() {
        println!("No domains currently blocked");
    } else {
        println!("Blocked Domains:");
        for domain in &list {
            println!("-{}", domain);
        }
    }
    Ok(())
}