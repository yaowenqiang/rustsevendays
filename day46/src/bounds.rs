use std::fmt::Display;
use std::fmt::Debug;
fn printer<T: Display>(t: T) {
    println!("{}", t);
}
trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}
#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Trangle { length: f64, height: f64 }

fn area<T: HasArea>(t: &T) -> f64 {t.area()}

struct S<T: Display>(T);
fn main() {
    //let s = S(vec![1]);
    let rectangle = Rectangle{ length: 3.0, height: 4.0};
    let _triangle = Trangle{ length: 3.0, height: 4.0};
    print_debug(&rectangle);
    print_debug(&_triangle);
    println!("Area: {}", area(&rectangle));
    //println!("Area: {}", area(&_triangle));


}
