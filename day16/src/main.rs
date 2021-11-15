extern crate redis;
use redis::{Client, Commands, Connection, RedisResult};
use std::collections::HashSet;
type UserId = u64;

fn main() {
    println!("24 days of Rust - redis (day 18)");
    let mut client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();
    let my_id: UserId = 1;
    let their_id: UserId = 2;

    add_friend(&mut conn,my_id, their_id).expect("friendship failed!");
}

fn add_friend(conn: &mut Connection, my_id: UserId, their_id: UserId) -> RedisResult<()> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    conn.sadd(my_key, their_id)?;
    conn.sadd(their_key, my_id)?;
    Ok(())
}
