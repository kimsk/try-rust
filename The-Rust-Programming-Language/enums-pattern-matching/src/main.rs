#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
    California,
    Tennessee,
    Texas
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    NotSupported
}

fn value_in_cents(coin: &Coin) -> (u8, Option<String>) {
    match coin {
        Coin::Penny => (1, None),
        Coin::Nickel => (5, None),
        Coin::Dime => (10, None),
        Coin::Quarter(state) => {
            let state = format!("{:?}", state);
            (25, Some(state))
        },
        _ => (0, None)
    }
}
// Rust’s enums are most similar to algebraic data types 
// in functional languages, such as F#, OCaml, and Haskell.

fn main() {
    let coins = [
        Coin::Dime,
        Coin::Quarter(UsState::Tennessee),
        Coin::NotSupported,
        Coin::Penny,
        Coin::Nickel,
        Coin::NotSupported
    ];
    let mut sum = 0;
    for i in 0..coins.len() {
        let coin = &coins[i];
        let value = value_in_cents(coin);
        match value {
            (0, _) => println!("Not Supported!"),
            (i, None) => {
                sum += i;
                println!("value: {}", i);
            },
            (i, Some(state)) => {
                sum += i;
                println!("value: {} {}", i, state)
            }
        }
    }
    println!("SUMS: {}", sum);

    println!("{}", add2_safe_match(None));
    println!("{}", add2_safe_unwrap_or(None));
    println!("{}", add2_safe_if_let(None));
    println!("{}", add2_unsafe(None)); // panicked
}

fn add2_safe_match(num: Option<i32>) -> i32 {
    match num {
        None => 2,
        Some(num) => num + 2
    }
}

fn add2_safe_unwrap_or(num: Option<i32>) -> i32 {
    num.unwrap_or(0) + 2
}

fn add2_safe_if_let(num: Option<i32>) -> i32 {
    if let Some(num) = num {num + 2}
    else {2}
}

// export RUST_BACKTRACE = 1

fn add2_unsafe (num: Option<i32>) -> i32 {
    num.unwrap() + 2
}
