extern crate reqwest;
//use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    let mut response = reqwest::get("https://httpbin.org/status/418")
        .expect("Failed to send request");
    println!("{}", response.status());

    for header in response.headers().iter() {
        println!("{}: {}", header.name(), header.value_string());
    }

    let mut buf = String::new();
    response.read_to_string(&mut buf).expect("Failed to read response");
    println!("{}", buf);
    */

    let response = reqwest::blocking::get("https://httpbin.org/status/418")?;
    println!("{}", response.status());

    for (key, value) in response.headers().iter() {
        println!("{:?}: {:?}", key, value);
    }

    let content = response.text().expect("Failed to read response");
    println!("{}", content);
    Ok(())
}
