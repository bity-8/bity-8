extern crate rand;

use std::cmp::Ordering;
use rand::Rng;

fn main() {
    loop {
        let secret_number = rand::thread_rng().gen_range(1, 101);
        println!("The secret number is {}!", secret_number);
        println!("Please input your guess.");
        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u64 = guess.trim().parse()
            .expect("noooo");

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
