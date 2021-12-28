fn foo<T> (arg: T) {

}
#[derive(Debug)]
struct A;
#[derive(Debug)]
struct Single(A);
#[derive(Debug)]
struct SingleGen<T>(T);

fn main() {
    let _s = Single(A);
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    let _char = SingleGen('a');
    println!("{:?}", _s);
    println!("{:?}", _t);
    println!("{:?}", _i32);
    println!("{:?}", _char);
}
