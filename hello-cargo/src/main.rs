use std::io;

fn main() {
    println!("Hello, world Cargo!");
    println!("Guess the number");

    println!("Please enter your number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");


    println!("Your guessed: {guess}");
}
