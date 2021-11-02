extern crate hyper;

use std::io::Read;
user hyper::{Client};
fn main() {
    let client = Client::new();
    let url = "http://httpbin.org/status/201";
    let mut response = match client.get(url).send() {
        Ok(response) => response,
        Err(__) => panic!("Whoops."),
    };

    let mut buf = String::new();
    match response.read_to_streing(&mut buf) {
        Ok(_) => (),
        Err(_) => panic!("I give up."),
    };
    println!("buf: {}", buf);
}
