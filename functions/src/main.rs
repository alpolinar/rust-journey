use rand::Rng;
use std::io;

fn main() {
    println!("Enter a greeting:");
    let mut words = String::new();
    io::stdin()
        .read_line(&mut words)
        .expect("Failed to read line");
    say(&words);

    let x = gen_number(100);
    let y = {
        let x = x + 1;
        let add_number = gen_number(10);
        println!("add_number: {add_number}");
        x + add_number
    };

    let val = add(&x, &y);
    println!("Adding {x} and {y}  = {val}");
    let random_month = get_month();
    println!("Random month: {random_month}");
}

fn say(words: &String) {
    println!("You said: {words}");
}

fn add(num_1: &u32, num_2: &u32) -> u32 {
    num_1 + num_2
}

fn get_month() -> &'static str {
    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let index = gen_number(12) as usize;
    months[index]
}

fn gen_number(range: u32) -> u32 {
    rand::thread_rng().gen_range(0..range)
}
