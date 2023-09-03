use std::io;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    let mut guess = String::new();

    loop {
        println!("Please input your guess.");

        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        if secret_number == guess {
            println!("You win!");
            break;
        } else {
            println!("Sorry, try again.");
        }
    }
}