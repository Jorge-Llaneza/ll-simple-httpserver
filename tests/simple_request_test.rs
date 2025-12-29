use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;
use std::sync::mpsc;
use ll_simple_httpserver;
use ll_simple_httpserver::http::parser::{HttpRequest, Verb};
use ll_simple_httpserver::http::uri::Uri;

#[test]
fn parse_request() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {server(tx)});

    let assigned_port = rx.recv().unwrap();

    let mut stream = TcpStream::connect(assigned_port).unwrap();
    stream.write_all(b"GEt / HTTp/1.0\r\nHost: localhost\r\n\r\n").unwrap();

    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = str::from_utf8(&buffer).unwrap().trim_end_matches("\0");
    if response != "HTTP/1.0 200 OK\r\n\r\n" {
        panic!("Wrong response: {}", response);
    }
}

fn server(tx: mpsc::Sender<SocketAddr>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().expect("Could not get address");

    tx.send(addr).unwrap();

    for stream in listener.incoming() {
        let mut  stream = stream.unwrap();
        let mut buff = [0; 1024];
        stream.read(&mut buff).unwrap();
        let contents = std::str::from_utf8(&buff).unwrap();
        let parts = contents.split_once("\r\n\r\n").unwrap();
        let header = parts.0;
        let request = ll_simple_httpserver::http::parser::HttpRequest::from_str(header).unwrap();

        let headers = request.get_headers();
        assert_eq!(headers.len(), 1); //TODO headers count should be 1

        let request_line = request.get_request_line();

        if request_line.verb() == &Verb::Get &&
            request_line.url() == &Uri::from_str("/").unwrap() &&
            request_line.protocol() == "http/1.0" {
            stream.write_all(b"HTTP/1.0 200 OK\r\n\r\n").unwrap();
        } else {
            stream.write_all(b"HTTP/1.0 500 INTERNAL SERVER ERROR\r\n\r\n").unwrap();
        }

    }
}
fn str_to_http_bytes(input: &str) -> Vec<u8> {
    input
        .replace("\r\n", "\n") // Step 1: Normalize all line endings to LF (\n)
        .replace("\n", "\r\n") // Step 2: Convert all LF to CRLF (\r\n)
        .into_bytes()          // Step 3: Consume the String and return the bytes
}
#[test]
fn all_headers_parsing_test() {
    let raw_request = r#"POST /api/v1/update HTTP/1.1
Host: localhost:8080
Accept: application/json
Authorization: Basic dXNlcjpwYXNz
Referer: https://google.com/
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64)
Content-Length: 27
Content-Type: application/json
X-Custom-Debug-ID: 99b1-az22

{"status": "testing-rust"}"#;

    // Convert to the wire-ready Vec<u8>
    let request_buffer = str_to_http_bytes(raw_request);

    let request_str = str::from_utf8(&request_buffer).unwrap();

    let parsed_request = HttpRequest::from_str(request_str).unwrap();
    assert_eq!(parsed_request.get_headers().len(), 8);
    assert_eq!(parsed_request.get_meth);
}