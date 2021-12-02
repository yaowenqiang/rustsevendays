extern crate cursive;
use cursive::Cursive;
use cursive::views::{Dialog, TextView};

fn main() {
    /*
    let mut app = Cursive::new();
    app.add_layer(Textview::new("Hello Rust"));
    app.add_global_callback('q', |a | a.quit());
    app.run();
    */

    let mut siv = cursive::default();
    siv.add_layer(Dialog::around(TextView::new("Hello Dialog"))
                  .title("Cursive")
                  .button("Quit", | s | s.quit()));

    siv.run();
}
