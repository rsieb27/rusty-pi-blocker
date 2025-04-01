use std::net::UdpSocket;
use std::sync::{Arc, Mutex};

use crate::blocklist;
use crate::logger;

pub fn start_dns_server() {
    let socket = UdpSocket::bind("0.0.0.0:53").expect("failed tp bind to port 53");
    println!("DNS sever running on port 53...");

    let blocklist = Arc::new(Mutex::new(blocklist::load_blocklist()));

    loop {
        let mut buf = [0; 512];
        let (amt, src) = socket.recv_from(&mut buf).expect("failed to receive data");

        // TODO: properly parse dns query to get the requested domain
        let domain = "example.com".to_string();

        let is_blocked = blocklist.lock().unwrap().contains(&domain);

        if is_blocked {
            //log the block
            logger::log_blocked_domain(&domain, &src.ip().to_string());
            //respond with nxdomain
            let nxdomain = generate_nxdomain_reponse(&buf[..amt]);
            socket.send_to(&nxdomain, src).unwrap();
            println!("blocked domain: {}", domain);
        } else {
            //forward to upstream dns
            forward_dns_request(&buf[..amt], src, &socket);
        }
    }
}

fn generate_nxdomain_reponse(_request: &[u8]) -> Vec<u8> {
    //build a proper dns reponse indicating NXDOMAIN
    //this is simplified. real dns repsonse logic trust dns or manual packet building
    vec![]
}

fn forward_dns_request(request: &[u8], src: std::net::SocketAddr, socket: &UdpSocket) {
    let upstream_dns = "8.8.8.8:53";
    let upstream_socket = UdpSocket::bind("0.0.0.0:0").expect("failed to bind local socket");
    upstream_socket.send_to(request, upstream_dns).unwrap();

    let mut response_buf = [0;512];
    let (resp_size, _) = upstream_socket.recv_from(&mut response_buf).unwrap();
    socket.send_to(&response_buf[..resp_size], src).unwrap();
}