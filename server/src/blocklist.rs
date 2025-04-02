use std::fs;
use std::path::Path;
use eyre::{Result, eyre};

use crate::error::AdBlockerError;

const BLOCKLIST_FILE: &str = "blocked_domains.txt";

pub fn load_blocklist() -> Result<Vec<String>> {
    if !Path::new(BLOCKLIST_FILE).exists() {
        return Ok(vec![]); //no file yet = empty blocklist
    }
    
    let contents = fs::read_to_string(BLOCKLIST_FILE)?; //read file
    Ok(contents.lines().map(String::from).collect()) //split ijnto lines

}

pub fn add_domain(domain: &str) -> Result<()> {
    let mut list = load_blocklist()?; //get current blocklist

    if list.contains(&domain.to_string()) {
        //domain alreafy in blocklist
        println!("Domain '{}' is already in blocklst", domain);
        return Ok(());
    }

    list.push(domain.to_string());
    fs::write(BLOCKLIST_FILE, list.join("\n"))?;
    println!("Domain '{}' added to blocklst", domain);

    Ok(())
}

pub fn remove_domain(domain: &str) -> Result<()> {
    let mut list = load_blocklist()?;
    let initial_len = list.len();

    list.retain(|d| d != domain); //remove the matching domain

    if list.len() == initial_len {
        println!("Domain '{}' was not in blocklist", domain);
    } else {
        fs::write(BLOCKLIST_FILE, list.join("\n"))?;
        println!("DOmain '{}' removed from blocklist", domain);
    }
    Ok(())
}

pub fn list_blocked_domains() -> Result<()> {
    let list = load_blocklist()?;
    if list.is_empty() {
        println!("No domains currently blocked");
    } else {
        println!("Blocked Domains:");
        for domain in list {
            println!("-{}", domain);
        }
    }

    Ok(())
}