use std::io;
extern crate math;
use math::sub;

fn main() {
    println!("Guess the number!!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };
    
    println!("You guessed: {guess}");

    let m2 = sub::sub_two(guess);
    println!("Minus two: {m2}")
}
