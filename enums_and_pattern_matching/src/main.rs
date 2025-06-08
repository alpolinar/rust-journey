#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new!"))
        }
    } else {
        None
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn plug_one_convert(x: u8) {
    let config_max = Some(3u8);
    if let Some(x) = config_max {
        println!("The maximum is configured to be {x}");
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {
    println!("move {num_spaces} spaces")
}
fn reroll() {
    println!("reroll")
}

fn use_catch_all(x: u8) {
    match x {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn catch_all(x: u8) {
    match x {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn main() {
    let val = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("the coin is {val}");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("some is {six:?}");
    println!("none is {none:?}");
    let roll: u8 = 9;
    use_catch_all(roll);
    catch_all(roll);
    let desc = describe_state_quarter(Coin::Quarter(UsState::Alabama));
    println!("{desc:?}")
}
