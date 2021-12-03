extern crate tera;

use tera::{Context,Tera};
const LIPSUM: &'static str = "static string";


fn main() {
    let tera = match Tera::new("templates/**.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    let mut ctx = Context::new();
    ctx.insert("title", &"hello world");
    ctx.insert("content", &LIPSUM);
    ctx.insert("todos", 
            &vec!["buy milk", "walk the dog", "write about tera"]);
    let rendered = tera.render("index.html", &ctx).expect("Failed to render template");
    println!("{}", rendered);
}
