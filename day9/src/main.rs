extern crate num;
extern crate tau;
use num::complex::{Complex, Complex64};

fn main() {
    println!("t = {}", tau::TAU);
    let radius: f64 = 15.0;
    println!("circle circumference = t * r = {}", tau::TAU * radius);
    let c: Complex64 = Complex::from_polar(1.0, tau::TAU);
    println!("Euler's identity: exp(i * t) = {}", c);
    println!(
        "trigonometry: sin(t) = {}, cos(t) = {}",
        tau::TAU.sin(),
        tau::TAU.cos()
    );
}
