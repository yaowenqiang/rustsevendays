extern crate redis;
extern crate rand;
use redis::{Client, Commands, Connection, RedisResult};
use std::collections::HashSet;
type UserId = u64;

type Leaderboard = Vec<(String, u32)>;

fn main() {
    println!("24 days of Rust - redis (day 18)");
    let mut client = Client::open("redis://127.0.0.1/").unwrap();
    let mut conn = client.get_connection().unwrap();
    let my_id: UserId = 1;
    let their_id: UserId = 2;

    add_friend(&mut conn,my_id, their_id).expect("friendship failed!");

    let answer: i32 = conn.get("answer").unwrap();
    println!("Answer: {}", answer);

    for i in 1 ..10u64 {
        add_friend(&mut conn, i, i + 2).expect("Friendship failed :(");
    }

    println!("You have {} friends in common.", 
             friends_in_common(&mut conn, 2, 3).
             map(|s| s.len()).unwrap_or(0));

    let players = vec!["raynor", "kerrigan", "mengsk", "zasz", "tassadar"];
    for player in &players {
        let score = rand::random::<u32>() % 1000;
        add_score(&mut conn, *player, score).expect("Nuclear launch detected");
    }

    show_leaderboard(&mut conn, 3);
}

fn add_friend(conn: &mut Connection, my_id: UserId, their_id: UserId) -> RedisResult<()> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    conn.sadd(my_key, their_id)?;
    conn.sadd(their_key, my_id)?;
    Ok(())
}

fn friends_in_common(conn: &mut Connection, my_id: UserId, their_id: UserId) -> RedisResult<HashSet<UserId>> {
    let my_key = format!("friends:{}", my_id);
    let their_key = format!("friends:{}", their_id);
    conn.sinter((my_key, their_key))
}

fn add_score(conn: &mut Connection, user_name: &str, score: u32) -> RedisResult<()> {
    conn.zadd("leaderboard", user_name, score)
}

fn show_leaderboard(conn: &mut Connection, n: isize) {
    let result :RedisResult<Leaderboard> = conn.zrevrange_withscores("leaderboard", 0, n-1);
    match result {
        Ok(board) => {
            println!("---=== Top {} players ===---", n);
            for (i,(username, score)) in board.into_iter().enumerate() {
                println!("{:<5} {:^20} {:>4}", i+1, username, score);
            }
        },
        Err(_) => println!(" Failed to fetch leaderboard."),
    }
}
