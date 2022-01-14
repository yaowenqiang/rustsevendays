#[derive(Debug)]
struct Borrowed<'a>(&'a i32);


#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}
#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}
fn main() {
    let mut x = 10;
    let y = 15;
    let mut single = Borrowed(&x);
    let mut double = NamedBorrowed { x: &x, y: &y };
    let mut reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("X is borrowed in {:?}", single);
    println!("X and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);

    single.0 = &20;
    double.x = &30;
    reference = Either::Num(40);

    println!("X is borrowed in {:?}", single);
    println!("X and y are borrowed in {:?}", double);
    println!("x is borrowed in {:?}", reference);
    println!("y is *not* borrowed in {:?}", number);
    println!("x in {:?}", x);
}

