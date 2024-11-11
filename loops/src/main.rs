fn main() {
    counter();
    loop_with_label();
    loop_with_while();
    loop_wih_while_index();
    loop_with_for();
    loop_with_for_range();
}

fn counter() {
    println!("counter");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // add return value after break
            break counter * 2;
        }
    };
    println!("The result is {result}");
}

fn loop_with_label() {
    println!("loop_with_label");
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}")
}

fn loop_with_while() {
    println!("loop_with_while");
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn loop_wih_while_index() {
    println!("loop_wih_while_index");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn loop_with_for() {
    println!("loop_with_for");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn loop_with_for_range() {
    println!("loop_with_for_range");
    for number in (1..4).rev() {
        println!("{number}!")
    }
    println!("LIFTOFF!!!");
}
