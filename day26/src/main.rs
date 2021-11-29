#[macro_use]
extern crate nom;
fn main() {
    let first_line = b"GET /home/ HTTP/1.1\r\n";
    named!(parse_method_v1, alt!(tag!("GET") | tag!("POST")));
    println!("{:?}", parse_method_v1(&first_line[..]));
}
