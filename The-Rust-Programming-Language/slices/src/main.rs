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
    //s.clear(); // not compiled, previous line borrowed as immutable already
    println!(
        "{hello} {world} {first_word}",
        hello=hello, world=world, first_word=word);

    let word = first_word_idx(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    //println!("{}", &s[..word]); // compiled but panicked

    string_slices_as_parameters();
    other_slices();
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

fn string_slices_as_parameters() {

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    
        &s[..]
    }

    let my_string: String = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    println!("{} &my_string[..]: &str", &my_string[..]);
    println!("{}", word);

    // https://doc.rust-lang.org/std/string/struct.String.html#deref
    // Strings implement Deref<Target=str>, and so inherit all of str's methods. In addition, this means that you can pass a String to a function which takes a &str by using an ampersand (&):

    //  Deref coercion
    // This will create a &str from the String and pass it in. This conversion is very inexpensive, and so generally, functions will accept &strs as arguments unless they need a String for some specific reason.
    let word = first_word(&my_string);
    println!("{} &my_string: &String", word);
    println!("{}", word);

    let my_string_literal: &str = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    // let x = my_string_literal[..]; // not compiled, size unknown at compiled time
    println!("{} &my_string_literal[..]: &str", word);
    println!("{}", word);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{} my_string_literal: &str", word);
    println!("{}", word);
    let x: &str = my_string_literal;
    println!("{}", x);

    let y: &&str = &my_string_literal;
    println!("{}", y);
}

// Another data type that does not have ownership is the slice. Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection.
fn other_slices() {
    let mut a = ['a', 'b', 'c'];
    println!("{:?}", a);
    let a0 = a[0]; // cloned
    a[0] = 'x';
    println!("{} {:?}", a0, a);

    let aa = [1, 2, 3];
    //let aaa = aa[..]; // not compiled, size unknown at compiled time
    let aaa = &aa[..]; // compiled, slice is fine
}