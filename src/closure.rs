// Fn Traits
// Fn: can rn any number of times, only closing over immutable bindings
// FnMut: Can run any number of times, closing over mutable and immutable bindings
// FnOnce: Can run once, taking ownership of captured bindings

use std::thread;

/*
fn <T: Fn(i32) -> i32 > f1() -> T {
    let f = |x| { x + 1}
    return f;
}

fn f2() -> Box<dyn Fn(i32) -> i32 {
    
}
*/

fn main() {
    let closure1 = |x| { x + 1};
    let val = 10;
    let closure2 = |x| { x + val};
    println!("{}", closure1(2));
    println!("{}", closure2(2));

    let handle = thread::spawn(|| {
        println!("From a thread!");
    });
    println!("Before a thread!");
    handle.join();
}