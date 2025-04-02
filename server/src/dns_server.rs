use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use eyre::{Result, eyre};

use crate::blocklist;
use crate::logger;
use crate::error::AdBlockerError;

pub fn start_dns_server() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:53")
        .map_err(|e| eyre!(AdBlockerError::UdpSocketError(e.to_string())))?; //listen on dns port
    println!("DNS sever running on port 53...");

    //unwrap the blocklist to Vec<String>
    let blocklist_data = blocklist::load_blocklist()?;
    let blocklist = Arc::new(Mutex::new(blocklist_data));

    let mut buf = [0; 512]; //standard dns packet size

    loop {
        //wait for dns request
        let (amt, src) = socket
            .recv_from(&mut buf)
            .map_err(|e| eyre!(AdBlockerError::UdpSocketError(e.to_string())))?;

        let domain = "example.com".to_string(); //placeholder domain --- actaul parsing later

        let list = blocklist.lock().map_err(|_| eyre!("Blocklist lock is poisoned"))?;

        if list.contains(&domain) {
            //log the block
            let _ = logger::log_blocked_domain(&domain, &src.ip().to_string()); //log domain
            //respond with nxdomain
            let nxdomain = generate_nxdomain_response(&buf[..amt]);
            socket
                .send_to(&nxdomain, src)
                .map_err(|e| eyre!(AdBlockerError::UdpSocketError(e.to_string())))?; //say "no such domain"
            println!("blocked domain: {}", domain);
        } else {
            //forward to real dns
            forward_dns_request(&buf[..amt], src, &socket)?;
        }
    }
}

fn generate_nxdomain_response(_request: &[u8]) -> Vec<u8> {
    //return a minimalr dns packet indicating NXDOMAIN for conciseness return an empty vec; change later
    vec![]
}

fn forward_dns_request (
    request: &[u8], 
    src: std::net::SocketAddr, 
    socket: &UdpSocket
) -> Result<()> {
    let upstream_dns = "8.8.8.8:53";

    let upstream_socket = UdpSocket::bind("0.0.0.0:0")
        .map_err(|e| eyre!(AdBlockerError::UdpSocketError(e.to_string())))?;
    upstream_socket
        .send_to(request, upstream_dns)
        .map_err(|e| eyre!(AdBlockerError::DnsForwardError(e.to_string())))?;

    let mut response_buf = [0;512];
    let (resp_size, _) = upstream_socket
        .recv_from(&mut response_buf)
        .map_err(|e| eyre!(AdBlockerError::DnsForwardError(e.to_string())))?;

    socket
        .send_to(&response_buf[..resp_size], src)
        .map_err(|e| eyre!(AdBlockerError::DnsForwardError(e.to_string())))?;

    Ok(())
}