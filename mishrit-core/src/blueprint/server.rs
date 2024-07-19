use crate::config;
use crate::config::HttpVersion;
use crate::valid::{Valid, ValidationError, Validator};
use std::net::{AddrParseError, IpAddr};

#[derive(Clone, Debug)]
pub struct Server {
    pub hostname: IpAddr,
    pub port: u16,
    pub http: Http,
    pub worker: usize,
}

#[derive(Clone, Debug)]
pub enum Http {
    HTTP1,
}

impl TryFrom<&config::Server> for Server {
    type Error = ValidationError<String>;

    fn try_from(server: &config::Server) -> Result<Self, Self::Error> {
        let http_server = match server.get_version() {
            HttpVersion::HTTP2 => Valid::fail("HTTP/2 not supported yet.".to_string()),
            _ => Valid::succeed(Http::HTTP1),
        };

        validate_hostname(server.get_hostname())
            .fuse(http_server)
            .fuse(Valid::succeed(server.get_port()))
            .fuse(Valid::succeed(server.get_workers()))
            .map(|(hostname, http, port, worker)| Server {
                hostname,
                port,
                http,
                worker,
            })
            .to_result()
    }
}

fn validate_hostname(hostname: &str) -> Valid<IpAddr, String> {
    if hostname == "localhost" {
        Valid::succeed(IpAddr::from([127, 0, 0, 1]))
    } else {
        Valid::from(hostname.parse().map_err(|e: AddrParseError| {
            ValidationError::new(format!("Parsing failed because of {}", e))
        }))
        .trace("server")
        .trace("hostname")
    }
}
