#[derive(Debug)]
pub struct User{
    name: String,
    age: i32,
    shoeSize: i32,
    height: i32,
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
