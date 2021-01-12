// https://blog.thoughtram.io/ownership-in-rust/
#![allow(unused_variables)]
#![allow(unused_mut)]

pub fn ownership_in_rust_fn() {
    let s: &str = "Have a nice day";
    let s: String = "Have a nice day".to_string();
    let names: Vec<String> = vec!["Pascal".to_string(), "Christoph".to_string()];

    let name = "Pascal".to_string();
    let a = name;
    let b = &a;
    greet(b);
    // let b = name; // name is invalid at this point
}

fn greet(name: &String) {
    println!("Hello, {}!", name);
}
