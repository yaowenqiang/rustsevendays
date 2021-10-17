// Existing Generic types
// Vec, HashMap, LinkedList, and so an can hold anything
// Smart pointers(Box, Arc, Rc, Mutex, and so on)
// Option and result types

use std::fmt::Display;

struct Tagged<T> {
    value: T,
    tag: String,
}

impl<T> Tagged<T> {
    fn tag(&self) -> String {
        self.tag.clone()
    }
}

// Dynamic Dispatch
// V-Table

fn show_all(v: Vec<&dyn Display>) {
    for item in v {
        println!("{}", item);
    }
}

fn main() {
    let mut vi: Vec<i32> = vec![20];
    let mut vs: Vec<&str> = vec!["Hi."];
    vs.push("this is fun.");
    vi.push(1234);
    vs.pop();
    println!("vi: {}", vi.len());

    let v = vec![&12 as &Display,
                 &"Hi." as &Display];
    show_all(v);
}