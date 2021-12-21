#[derive(PartialEq, PartialOrd)]
struct Centimeters(f64);
#[derive(Debug)]
struct Inches(i32);
impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(inches) = self;
        Centimeters(inches as f64 * 2.54)
    }
}

struct Seconds(i32);

fn main() {
    let foot = Inches(12);
    println!("One foot equls  {:?}", foot);
    let meter = Centimeters(100.0);

    let cmp = 
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        };
    println!("One foot is {} then one meter.", cmp);
}
