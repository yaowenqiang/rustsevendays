extern crate leftpad;
use leftpad::{left_pad, left_pad_char};

fn main() {
    println!("{}", left_pad("pad me", 20));
    println!("{}", left_pad("pad me again", 20));
    println!("{}", left_pad_char("tick", 20, '✓'));
}
