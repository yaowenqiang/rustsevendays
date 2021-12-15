use std::path::Path;

fn main() {
    let path = Path::new(".");
    let _display = path.display();
    let new_path = path.join("a").join("b");
    match new_path.to_str() {
        None => panic!("new path is not a valid utf-i sequence"),
        Some(s) => print!("new path is {}", s),

    }

}
