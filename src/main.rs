use rand::prelude::*;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("guess my ass!");

    let mut rng = thread_rng();
    let secret_number = rng.gen_range(1, 101);

    println!("ass number is: {}", secret_number);

    loop {
        println!("input your fucking guess...");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Failed to read");

        let guess: u32 = match guess.trim().parse() {
            Ok(69) => {
                println!("nice.");
                69
            }
            Ok(num) => num,
            Err(_) => {
                println!("that is not a number, dickwad!");
                continue;
            }
        };

        print!("{}? ", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too massiv"),
            Ordering::Equal => {
                println!("finally...");
                break;
            }
        }
    }
}
