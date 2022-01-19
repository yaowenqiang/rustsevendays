fn main() {
    let i = 3;
    {
        let borrow1 = &i;
        println!("borrow1: {}", borrow1);
    }

    {
        let borrow2 = &i;
        println!("borrow2: {}", borrow2);
    }

    {
        let x = 0;
        {
            //let y = 42;
            let y = 42;
            //x = &y;
        }
        println!("the value of 'x' is {}", x);
    }

    let string1 : String = String::from("abcd");
    {
        let string2 = "xyz";
        let result;
        {
            result = longest(string1.as_str(), string2);
            {
                println!("The longest string is {}", result);
            }
        }
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
