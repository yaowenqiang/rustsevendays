extern crate hyper;

use hyper::Client;
use std::io::Read;
fn main() {
    let client = Client::new();
    let url = "https://ifconfig.me/";
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(__) => panic!("Whoops."),
    };

    let mut buf = String::new();
    match response.read_to_string(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    println!("buf: {}", buf);
}
