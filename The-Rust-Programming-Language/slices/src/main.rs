#![allow(unused_variables)]
fn main() {
    let mut s = String::from("hello world");

    // use slices
    let hello = &s[..5];
    let world: &str = &s[6..];
    let str_lit: &str = "hello world";
    // &str is an immutable reference
    let word = first_word_str(&s[..]);
    let word = first_word_str(str_lit);


    let word = first_word(&s);
    println!(
        "{hello} {world} {first_word}",
        hello=hello, world=world, first_word=word);

    let word = first_word_idx(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_idx(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // found space
            return i;
        }
    }

    s.len()
}

fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}