use std::io;
use rand::Rng;
use std::cmp::Ordering;

pub fn main() {
    println!("Guess the number! Enter a number in range [1 - 100]:");

    let secret_num: u32 = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess:");
    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => print!("Too small! "),
            Ordering::Greater => print!("Too big! "),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        println!("Try again.");
    }
}
