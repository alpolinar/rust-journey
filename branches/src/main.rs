use std::io;
use std::process;

fn main() {
    println!("Enter a number between 1 to 100");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let num: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Error: {err}");
            process::exit(1);
        }
    };
    let divisible_by = divisible(num);

    println!("{divisible_by}");
}

fn divisible(num: u32) -> &'static str {
    if num % 4 == 0 {
        return "number is divisilbe by 4";
    }
    if num % 3 == 0 {
        return "number is divisible by 3";
    }
    if num % 2 == 0 {
        return "number is divisible by 2";
    }
    "number is not divisible by 4, 3, or 2"
}
