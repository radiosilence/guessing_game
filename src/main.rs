use rand::Rng;
use std::io::stdin;

fn main() {
    println!("Guess my ass!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input yo guess...");

    let mut guess = String::new();

    stdin().read_line(&mut guess).expect("Failed to read");

    println!("You guessed {}", guess);
}
