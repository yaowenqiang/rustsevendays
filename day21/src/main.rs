extern  crate envy;
#[macro_use]
extern crate serde;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Environment {
    lang: String,
}

use std::env;

fn main() {
    match env::var("LANG") {
        Ok(lang) => println!("Language code: {}", lang),
        Err(e) => println!("couldn't read LANG ({})", e),
    };

    match envy::from_env::<Environment>() {
        Ok(environment) => println!("Language code: {}", environment.lang),
        Err(e) => println!("couldn't read LANG ({})", e),

    }
}
