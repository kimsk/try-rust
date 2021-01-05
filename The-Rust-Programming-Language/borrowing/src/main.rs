#![allow(unused_variables)]
fn main() {
    let s1 = String::from("hello");

    // borrowing
    // & are references, 
    // and they allow you to refer to some value without taking ownership of it.
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // giving back
    let (s2, len) = calculate_length2(s1);
    println!("The length of '{}' is {}.", s2, len);

    let mut s = String::from("hello");

    change(&mut s);
    println!("after change, {}", s);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    let s2 = s;

    //s.push_str(", world");
    //^ `s` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn calculate_length2(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
