use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;
use std::thread;
use std::sync::mpsc;
use ll_simple_httpserver;

#[test]
fn parse_request() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {server(tx)});

    let assigned_port = rx.recv().unwrap();

    let mut stream = TcpStream::connect(assigned_port).unwrap();
    stream.write_all(b"GEt / HTTp/1.0\r\nHost: localhost\r\n\r\n").unwrap();

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
        let request = ll_simple_httpserver::http::parser::HttpRequest::from_str(contents).unwrap();

    }
}