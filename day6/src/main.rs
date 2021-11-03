extern crate rustc_serialize;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, Encoder};

fn main() {
    println!("{:?}", json::encode(&42));
    println!("{:?}", json::encode(&vec!["to", "be", "or", "not", "to", "bt"]));
}
