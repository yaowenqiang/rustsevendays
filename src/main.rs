extern crate rand;
use rand::Rng;
use std::collections::HashMap;
use std::env::args;
use std::ops::Add;


#[derive(Debug)]
pub struct Point{
    x:i32,
    y:i32,
}
impl Point {
    fn random() -> Self{
        let mut tr = rand::thread_rng();
        Point{
            x: tr.gen(),
            y: tr.gen(),
        }
    }
}
impl Add for Point {
    type  Output = Point;
    fn add(self, other:Point) -> Self::Output {
        Point{
            x:self.x + other.x,
            y:self.y + other.y,
        }
    }
}

#[derive(Debug)]
pub struct User{
    name: String,
    age: i32,
    shoeSize: i32,
    height: i32,
}

#[derive(Debug)]
pub struct Bed{
    size:i32,
    count:u32,
}

#[derive(Debug)]
pub enum Room{
    Kitchen(i32),
    Bedroom(Bed),
    Lounge(i32, String),
}

impl User {
    //fn simple_string(&mut self) {
    fn simple_string(&self) -> String {
        format!("{} - {} - {}cm shoe: {}", self.name, self.age, self.height, self.shoeSize)
    }
    fn grow(&mut self, h:i32) {
        self.height += h;
    }
    fn die(self) {
        println!("Dead {} ", self.simple_string());
    }
}

fn main() {
    //traits

    let a = Point{x:3, y:5};
    let b = Point{x:30, y:50};
    let c = a + b;
    println!("c = {:?}", c);
    let d = Point::random();
    println!("d = {:?}", d);



    for a in args() {
        //assignment
        if let Some(c) = a.chars().next() {
            match c {
                'w' | 'W' => println!("Hello {}", a),
                _ => {}
            }
        }
    }

    //Result and Option types

    let mut hm = HashMap::new();
    hm.insert(3, "Hello");
    hm.insert(5, "World");
    let r = match hm.get(&3) {
        Some(v) => v,
        _ => "Nothing",
    };
    println!("{}", r);
    
    let r = hm.get(&5).unwrap();
    println!("{}", r);
    let r1 = hm.get(&4).unwrap_or(&"NoString");
    println!("{}", r1);

    match "3".parse::<i32>() {
        Ok(v) => println!("OK - {}", v),
        Err(e) => println!("Error - {}", e),
    }

    match "abc".parse::<i32>() {
        Ok(v) => println!("OK - {}", v),
        Err(e) => println!("Error - {}", e),
    }

    match get_arg(3) {
        Ok(v) => println!("OK - {}", v),
        Err(e) => println!("Error - {}", e),
    }

    //enum

    //let t = Room::Kitchen(4);
    use self::Room::*;
    //let t = Room::Bedroom(Bed{size:50, count:2});
    //let t = Bedroom(Bed{size:50, count:2});
    let t = Kitchen(4);
    let l = Lounge(5, "big".to_string());
    println!("Hello from the {:?}", t);
    /*
    match t {
        Room::Kitchen(n) => println!("The room is a Kitchen with {} rooms", n),
        d => println!("{:?}", d),
    }
    */
    let v = match t {
        Room::Kitchen(n) => n,
        _ => 0,
    } + 10;
    println!("v is {}", v);

    if let Kitchen(n) = t {
        println!("Its a Kitchen with {} cupboards", n);
    }

    if let Lounge(n, s) = l {
        println!("Its a {} Lounge with {} cupboards", n, s);
    }
    let mut u = User{
        name: "Jack".to_string(),
        age: 33,
        height: 255,
        shoeSize: 10,
    };
    println!("{:?}",u);
    println!("User is {}",u.simple_string());
    u.grow(10);
    println!("User is {}",u.simple_string());
    u.die();
    //u.die();
    //u.grow(10);

    let b  = highest(4, 2, 8);
    let s = format!("{} is highest", b);
    let o = other(1, 2);
    println!("{}", s);
    println!("{} is other", o);
    loop_to_10();
    array_loop();
    array_loop2();
    //strings

    let mut s = String::from("hello 中国");
    println!("the length of s is {}", s.len());
    println!("the number l of s is {}", count_l(&s));
    //let s = "hello 中国";
    //let mut s = "hello 中国";
    s.push_str("herr");
    for c in s.chars() {
        println!("{}", c);
    }
    for (i, c) in s.chars().enumerate() {
        println!("{} = {}", i, c);
    }
    for (i, c) in s.char_indices(){
        println!("{} = {}", i, c);
    }
    for c in s.bytes() {
        println!("{}", c);
    }
}

fn highest(a:i32, b: u32, c: i8) -> i32 {
    let mut res = a;
    if b as i32 > res {
        res = b as i32;
    }
    if c as i32 > res {
        res = c as i32;
    }
    res
}

fn other(a:i32, b: i32) -> i32 {
    let mut c = a + b;
    c = c % 4;
    c = c / 2;
    c += 1;
    c
}

fn loop_to_10(){
    for n in 0..10 {
        println!("hello {}", n);
    }
}

fn array_loop() {
    let mut v = Vec::new();
    v.push(4);
    v.push(7);
    v.push(7);
    v.push(7);
    v.push(7);
    v.push(7);
    for n in v {
        println!("{}", n);
    }

}

fn array_loop2() {
    let v = vec![1,2,3,4];
    'outer: for i in 0..10 {
        for n in &v {
            if i+n >= 10 {
                //continue;
                break 'outer;
            }
            if n % 2 == 0 {
                //continue;
                break;
            }
            println!("{}", n);
        }

    }

}

fn count_l(s:&str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        if c == 'l' {
            res +=1;
        }
    }
    res
}

fn get_arg(n:usize) -> Result<String, String> {
    for (i, a) in args().enumerate() {
        if i == n {
            return Ok(a);
        }
    }
    Err("Not enough Args".to_string())
}