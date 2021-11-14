extern crate redis;
use redis::{Client, Commands, Connection, RedisResult};

fn main() {
    println!("24 days of Rust - redis (day 18)");
    let client = Client::open("redis://127.0.0.1/").unwrap();
    let conn = client.get_connection().unwrap();
}
