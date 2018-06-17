use std::io::prelude::*;
use std::net::TcpListener;

fn listen() -> Result<(), ()> {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        println!("new client connected !");
        let mut buf: [u8; 1000] = [0u8; 1000];
        stream.unwrap().read(&mut buf);
        println!("received {:?}", buf.to_vec());
    }
    Ok(())
}

fn main() {
    listen();
}