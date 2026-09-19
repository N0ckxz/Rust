// This is the prelude (rust term)
use std::io;

// Rng library that we got through cargo (SO COOOL)
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {secret_number}");

    println!("Please input your guess.");

    // By default variables are immutable in rust, unless
    // the mut flag is added
    // --------------------------------------------------
    // let is the statement to declare a variable
    let mut guess = String::new();

    io::stdin()
        // & its a REFERENCE, so, the address of the data
        .read_line(&mut guess)
        .expect("Failed to read line :(");

    println!("You guessed: {guess}");
}
