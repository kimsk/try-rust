fn main() {
    let s = String::from("hellno");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // move occurs
    // s can't be used
    // println!("{}", s);

    let s2 = String::from("hello");
    takes_ownership(s2.clone());
    println!("s2 is cloned! {}", s2);

    let s3 = takes_and_gives_back(s2);
    println!("s2 is back in s3! {}", s3);
    // s3 is ok, but s2 ownership is gone
    // println!("s2 is cloned! {}", s2);

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}