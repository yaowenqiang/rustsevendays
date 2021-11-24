extern  crate envy;
#[macro_use]
extern crate serde;
extern crate dotenv;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Environment {
    lang: String,
}

#[derive(Deserialize, Debug)]
struct Mailerconfig {
    email_backend: String,
    email_from: String,
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

    dotenv::dotenv().expect("Failed to read .env file");
    println!("Email backend: {}", env::var("EMAIL_BACKEND").expect("EMAIL_BACKEND not found!"));

    match envy::from_env::<Mailerconfig>() {
        Ok(config) => println!("{:?}", config),
        Err(e) => println!("Couldn't read mailer config ({})", e),
    };

}
