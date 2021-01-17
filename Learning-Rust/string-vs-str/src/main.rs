fn main() {
    let mut my_name: String = "Karlkim".to_string();
    println!("my_name {}", my_name);
    my_name.push_str(" SK");

    greet_string(&my_name);
    let name: &str = &my_name[..7];
    greet_str(name);

    my_name = greet_and_mutate_str(my_name);
    let name2: &str = &my_name;
    greet_str(name2);

    let ping: &str = "Ping";
    // &ping points to pre-allocated read-only memory with "Ping"
    // println!("*ping {}", *ping); // doesn't have a size known at compile-time
    println!("ping \t{}", ping); // dereferencing implicitly
    println!("&ping \t{}", &ping); // dereferencing implicitly
    println!("&&ping \t{}", &&ping); // dereferencing implicitly
    println!("&&&ping {}", &&&ping); // dereferencing implicitly
    println!("*&ping \t{}", *&ping); // dereferencing explicitly

}

fn greet_str(name: &str) {
    println!("Hello, {}!", name);
}

fn greet_string(name: &String) {
    println!("Hello, {}!", name);
}

fn greet_and_mutate_str(mut name: String) -> String {
    name.push_str(" The Great!");
    name
}

// use &str if we don't need to mutate
