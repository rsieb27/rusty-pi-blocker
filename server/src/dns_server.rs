use std::net::UdpSocket;
use std::process;
use std::fs;
use std::sync::{Arc, Mutex};
use trust_dns_proto::op::Message;
use trust_dns_proto::serialize::binary::BinEncodable;
use eyre::{Result, eyre};

use crate::blocklist;
use crate::logger;
use crate::error::AdBlockerError;

pub fn start_dns_server() -> Result<()> {
    //bind to the udp socket to port 53
    let socket = UdpSocket::bind("0.0.0.0:53")
        .map_err(|e| eyre!(AdBlockerError::UdpSocketError(e.to_string())))?;

    //write the Process id to a file
    let pid = process::id();
    fs::write("dns.pid", pid.to_string())?;
    println!("DNS server running on port 53 (pid: {})", pid);

    //unwrap the blocklist to Vec<String>
    let blocklist_data = blocklist::load_blocklist()?;
    let blocklist = Arc::new(Mutex::new(blocklist_data));

    let mut buf = [0; 512]; //standard dns packet size

    loop {
        //wait for dns request
        let (amt, src) = socket
            .recv_from(&mut buf)
            .map_err(|e| eyre!(AdBlockerError::UdpSocketError(e.to_string())))?;

        //parse dns message
        let dns_message = match Message::from_vec(&buf[..amt]) {
            Ok(msg) => msg, 
            Err(_) => continue,
        };

        //get domain from the first query
        let domain = match dns_message.queries().first() {
            Some(query) => {
                //normalize domain
                query.name().to_utf8().to_lowercase().trim_end_matches('.').to_string()
            }
            None => continue, 
        };

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

pub fn generate_nxdomain_response(request: &[u8]) -> Vec<u8> {
    //if request too small return empty
    if request.len() < 12 {
        return vec![];
    }

    let mut response = Vec::new();

    //transaction id -- first 2 bytes from request
    response.extend_from_slice(&request[0..2]);

    //FLAGS:
    //QR=1 -- response
    //RD=1 -- recursion desired 
    //RA=1 -- recursion available
    //RCODE=3 -- nonexistent domain 
    response.push(0x81);
    response.push(0x83);

    //copy QDCOUNT from original request; bytes 4-6
    response.extend_from_slice(&request[4..6]);

    //ANCOUNT = 0; no answer
    response.push(0x00);
    response.push(0x00);

    //NSCOUNT  = 0; no answer
    response.push(0x00);
    response.push(0x00);

    //ASCOUNT = 0; no answer
    response.push(0x00);
    response.push(0x00);

    //copy the question section from the original request
    let mut offset = 12;

    //copy all QNAME bytes until it hits a 0
    while offset < request.len() && request[offset] != 0 {
        response.push(request[offset]);
        offset += 1;
    }

    //copy the 0 terminator for that domain
    if offset < request.len() {
        response.push(0);
        offset += 1;
    } else {
        //bail if the request is malformed
        return vec![];
    }

    //copy the QTYPE + QCLASS 
    if offset + 4 <= request.len() {
        response.extend_from_slice(&request[offset..offset+4]);
        offset += 4;
    } else {
        return vec![];
    }

    response

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