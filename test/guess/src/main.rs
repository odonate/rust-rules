use std::cmp::Ordering;
use std::io;
extern crate rand_core;
use rand_core::{RngCore, SeedableRng};

struct SimpleRng {
    state: u64,
}

impl SeedableRng for SimpleRng {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut state = 0u64;
        for &byte in seed.iter() {
            state = (state << 8) | (byte as u64);
        }
        SimpleRng { state }
    }

    fn seed_from_u64(seed: u64) -> Self {
        SimpleRng { state: seed }
    }
}

impl RngCore for SimpleRng {
    fn next_u32(&mut self) -> u32 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        (self.state >> 32) as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        for chunk in dest.chunks_mut(8) {
            let rand = self.next_u64();
            for (i, byte) in chunk.iter_mut().enumerate() {
                *byte = (rand >> (i * 8)) as u8;
            }
        }
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

fn main() {
    println!("Guess the number!");

    let mut rng = SimpleRng::seed_from_u64(12345); // A simple seed for demonstration
    let secret_number = rng.next_u32() % 100 + 1;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
