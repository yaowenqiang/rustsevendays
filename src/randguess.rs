extern crate rand;
use rand::random;
use std::io;
pub fn get_guess() -> u8 {
    loop {
        println!("Input guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("could not read from stdin!");
        println!("get input guets : {}", guess);

        match guess.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(e) => println!("could not understand input: {}", e),
        };
    }
}

pub fn handle_guess(guess: u8, correct: u8) -> bool {
    if guess < correct {
        println!("Too low.");
        false

    } else if guess > correct {
        println!("Too high.");
        false
    } else {
        println!("You got it!");
        true
    }
}