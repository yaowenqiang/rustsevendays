struct Foo {
    quax: i32,
    baz: String::new(),
    z: fuz,
}
struct fuz {
    zed: i32,
}



type Point = (f32, f32)

struct Point2D {
    x: f64,
    y: f64,
}

fn add_points(a:Point2D, b: Point2D) -> Point2D {
    Point2D {
        x: a.x + b.x,
        y: a.y + b.y
    }
}