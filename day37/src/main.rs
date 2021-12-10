extern crate lettre;
extern crate lettre_email;
use lettre::{EmailTransport, SmtpTransport};
use lettre_email::EmailBuilder;
use std::path::Path;

fn main() {
    let email = EmailBuilder::new()
        .from("jacky.yao@163.com")
        .to("jacky.yao@163.com")
        .subject("rust lettre crate test")
        .text("hello world!")
        .build()
        .unwrap();

    let mut mailer = SmtpTransport::builder_unencrypted_localhost()
        .unwrap()
        .build();

    let result = mailer.send(&email);

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Couldn't send email: {:?}", result);
    }
}
