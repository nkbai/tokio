//! A "tiny" example of HTTP request/response handling using transports.
//!
//! This example is intended for *learning purposes* to see how various pieces
//! hook up together and how HTTP can get up and running. Note that this example
//! is written with the restriction that it *can't* use any "big" library other
//! than Tokio, if you'd like a "real world" HTTP library you likely want a
//! crate like Hyper.
//!
//! Code here is based on the `echo-threads` example and implements two paths,
//! the `/plaintext` and `/json` routes to respond with some text and json,
//! respectively. By default this will run I/O on all the cores your system has
//! available, and it doesn't support HTTP request bodies.

#![warn(rust_2018_idioms)]

use futures::{SinkExt, StreamExt};

use native_tls;
use std::{env, error::Error, fmt, io};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_tls::TlsConnector;

use std::net::ToSocketAddrs;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let res = get(String::from("github.com")).await?;
    println!("{}", res);
    Ok(())
}
//解析url,分成两部分 ip:port和path
fn parse_url(url: String) -> Result<(bool, String, String), io::Error> {
    let mut is_tls = url.starts_with("https://");
    if !url.starts_with("http://") && !is_tls {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "only http url support",
        ));
    }
    let mut url = url.as_str();
    //跳过http://
    if is_tls {
        url = &url[8..url.len()]
    } else {
        url = &url[7..url.len()]
    }
    let pos = url.find('/');
    if pos.is_none() {
        return Ok((is_tls, unsafe { String::from(url) }, String::from("/")));
    }
    let pos = pos.unwrap();
    let (ip_port, path) = url.split_at(pos);
    Ok((is_tls, String::from(ip_port), String::from(path)))
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_url() {
        let u = parse_url(String::from("https://github.com.ipaddress.com/")).unwrap();
        assert_eq!(
            u,
            (
                true,
                String::from("github.com.ipaddress.com"),
                String::from("/")
            )
        );
        let u = parse_url(String::from("http://github.com.ipaddress.com/")).unwrap();
        assert_eq!(
            u,
            (
                false,
                String::from("github.com.ipaddress.com"),
                String::from("/")
            )
        );
        let u = parse_url(String::from("https://github.com.ipaddress.com/somepath")).unwrap();
        assert_eq!(
            u,
            (
                true,
                String::from("github.com.ipaddress.com"),
                String::from("/somepath")
            )
        );
        let u = parse_url(String::from("https://github.com.ipaddress.com")).unwrap();
        assert_eq!(
            u,
            (
                true,
                String::from("github.com.ipaddress.com"),
                String::from("/")
            )
        );
        let u = parse_url(String::from("tcp://github.com.ipaddress.com"));
        assert!(u.is_err());
    }
    #[tokio::test]
    async fn test_get() {
        assert!(true);
    }
}
//根据url,获取其地址对应的html内容
async fn get(domain: String) -> Result<String, Box<dyn Error>> {
    // First up, resolve google.com
    let ip_port = format!("{}:443", domain.clone());
    println!("ip_port={}", ip_port);
    let addr = ip_port.to_socket_addrs().unwrap().next().unwrap();
    println!("addr={}", addr);
    let socket = TcpStream::connect(&addr).await.unwrap();
    // Send off the request by first negotiating an SSL handshake, then writing
    // of our request, then flushing, then finally read off the response.
    let builder = native_tls::TlsConnector::builder();
    let connector = builder.build().unwrap();
    let connector = tokio_tls::TlsConnector::from(connector);
    let mut socket = connector.connect(domain.as_str(), socket).await?;
    println!("write sockeet");
    socket
        .write_all(b"GET /codeload.github.com HTTP/1.0\r\nHost:github.com.ipaddress.com\r\n\r\n")
        .await?;

    let mut data = Vec::new();
    socket.read_to_end(&mut data).await?;
    Ok(unsafe { String::from_utf8_unchecked(data) })
}
