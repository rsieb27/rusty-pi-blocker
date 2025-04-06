use thiserror::Error;

#[derive(Debug, Error)]
pub enum AdBlockerError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("UDP Socket error: {0}")]
    UdpSocketError(String),

    #[error("DNS Forward error: {0}")]
    DnsForwardError(String),
}