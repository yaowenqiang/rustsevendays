extern crate rayon;
use rayon::prelude::*;

fn semicircle(x: f64) -> f64 {
    (1f64 - x * x).sqrt()
}

fn integrate<F>(f: F, points: usize) -> f64
where
    F: Fn(f64) -> f64 + Sync,
{
    let delta = 1f64 / points as f64;
    (0..points)
        .map(|i| {
            let a = i as f64 * delta;
            delta * (f(a) + f(a + delta)) / 1f64
        })
        .sum()
}
fn parallel_integrate<F>(f: F, points: usize) -> f64
where
    F: Fn(f64) -> f64 + Sync,
{
    let delta = 1f64 / points as f64;
    (0..points)
        .into_par_iter()
        .map(|i| {
            let a = i as f64 * delta;
            delta * (f(a) + f(a + delta)) / 1f64
        })
        .sum()
}
fn main() {
    println!("sequential: {}", 4f64 * integrate(semicircle, 10_000_000));
    println!(
        "parallel  : {}",
        4f64 * parallel_integrate(semicircle, 10_000_000)
    );
}
