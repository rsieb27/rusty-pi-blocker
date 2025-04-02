use hamcrest2::prelude::*;
use rust_dns_adblocker::blocklist;
use std::fs;
use std::path::Path;
use std::io::Write;
use eyre::Result;

#[test] 
fn test_load_blocklist() -> Result<()> {
    let _ = fs::remove_file("blocked_domains.txt");

    let list = blocklist::load_blocklist()?;
    assert_that!(list.len(), eq(0));
    Ok(())
}

#[test]
fn test_add_domain() -> Result<()> {
    let _ = fs::remove_file("blocked_domains.txt");

    blocklist::add_domain("ads.example.com")?;
    let list = blocklist::load_blocklist()?;

    assert_that!(list.contains(&"ads.example.com".to_string()), is(true));
    assert_that!(list.len(), eq(1));

    Ok(())
}

#[test]
fn test_remove_domain() -> Result<()> {
    let mut file  = fs::File::create("blocked_domains.txt")?;
    writeln!(file, "ads.example.com")?;
    writeln!(file, "ads0.example.com")?;
    drop(file);

    blocklist::remove_domain("ads.example.com")?;
    let list = blocklist::load_blocklist()?;

    assert_that!(list.contains(&"ads.example.com".to_string()), is(false));
    assert_that!(list.contains(&"ads0.example.com".to_string()), is(true));
    assert_that!(list.len(), eq(1));

    Ok(())
}

#[test]
fn test_remove_domain_nonexistent() -> Result<()> {
    let _ = fs::remove_file("blocked_domains.txt")?;

    blocklist::remove_domain("zebrasAreNot.real")?;
    let list = blocklist::load_blocklist()?;

    assert_that!(list.len(), eq(0));

    Ok(())
}

#[test]
fn test_blocked_domains() -> Result<()> {
    let _ = fs::remove_file("blocked_domains.txt")?;
    fs::write("blocked_domains.txt", "ads.example.com\nads0.example.com\n")?;

    let all_domains  = blocklist::load_blocklist()?;

    assert_that!(all_domains.len(), eq(2));
    assert_that!(all_domains.contains(&"ads.example.com".to_string()), is(true));
    assert_that!(all_domains.contains(&"ads0.example.com".to_string()), is(true));

    Ok(())
}