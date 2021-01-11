// Rust’s enums are most similar to algebraic data types 
// in functional languages, such as F#, OCaml, and Haskell.

fn main() {
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
