extern crate rustc_serialize;
use rustc_serialize::json::{self, Encoder};
use rustc_serialize::Encodable;

fn main() {
    println!("{:?}", json::encode(&42));
    println!(
        "{:?}",
        json::encode(&vec!["to", "be", "or", "not", "to", "bt"])
    );
}
