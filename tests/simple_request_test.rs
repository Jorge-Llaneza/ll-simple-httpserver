use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;
use std::sync::mpsc;
use ll_simple_httpserver;
use ll_simple_httpserver::http::parser::Verb;
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
        assert_eq!(headers.len(), 0); //TODO headers count should be 1

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