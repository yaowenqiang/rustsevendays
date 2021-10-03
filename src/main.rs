fn main() {
    let b  = highest(4, 2, 8);
    let s = format!("{} is highest", b);
    let o = other(1, 2);
    println!("{}", s);
    println!("{} is other", o);
    loop_to_10();
    array_loop();
    array_loop2();
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
