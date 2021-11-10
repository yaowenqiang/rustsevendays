extern crate  nalgebra;
use nalgebra::{Matrix2,Vector2};

fn main() {
    let v = Vector2::new(0.0f64, 1.0);

    let rot = Matrix2::new(0.0f64, -1.0, 1.0, 0.0);

    println!("{:?}", rot * v);
}
