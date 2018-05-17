extern crate rand;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}!", secret_number);
    println!("Please input your guess.");

    let mut guess = String::new();

    std::io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    test("testing me");
}

fn test(var: &str) {
    println!("{}", var);
}
