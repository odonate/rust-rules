use std::io;
extern crate math;
// extern crate rand;
// use rand::thread_rng;

fn main() {
    println!("Guess the number!!");

    // let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // println!("The secret number is {secret_number}");

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

    let m2 = math::add::add_two(guess);
    println!("Minus two: {m2}")
}
