use msrs::request::dsl::{on, to};
use msrs::request::host::IpAddress::Localhost;
use msrs::request::template::HttpResponse;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

mod msrs;

#[test]
fn dsl_test() {
    let post_request =
        to(Localhost, 8080)
            .post().slash("api").slash("users")
            .param("id", "12")
            .param("forename", "robert")
            .param("name", "dupont")
            .content("prout")
            .request();
    println!("Sending request : {:?}", post_request);

    let get_request =
        to(Localhost, 8080)
            .get().slash("api").slash("users")
            .request();
    println!("Sending request : {:?}", get_request);

    let get_request_template =
        on().get()
            .slash("api").slash("users")
            .answer(|req| {
                println!("handler {:?}", req);
                HttpResponse {}
            });
    println!("template {:?}", get_request_template);
    println!("template response {:?}", get_request_template.response(get_request));
}

fn listen() -> Result<(), ()> {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap();

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5000").unwrap();

    let _ = stream.write(&[1]); // ignore the Result
    let _ = stream.read(&mut [0; 128]); // ignore this too
}
