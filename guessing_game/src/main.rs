use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guessing Game!");
    let secret_number = gen_number();

    println!("Select a number between 1-100");
    println!("secret number {}", secret_number);
    loop {
        println!("Enter your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        println!("Your guess {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
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

fn gen_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}
