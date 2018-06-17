use std::io::prelude::*;
use std::net::TcpStream;

fn write() {
    let mut stream = TcpStream::connect("127.0.0.1:5000").unwrap();

    let _ = stream.write(&[1]);
}

fn main() {
    write()
}