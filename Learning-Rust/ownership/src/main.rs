#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
mod emoji;
mod reference_in_rust;
mod ownership_in_rust_mod;
mod ownership_in_rust_folder;
mod other_mod;
// `mod` declaration will look for a file named `emoji.rs` or `other_mod/mod.rs` and will insert its contents inside modules under this scope


use emoji::emoji as emooji;

fn main() {
    let mut x = String::from("HELLO");
    let mut a = x; // "HELLO" was moved to a
    let mut b = a.clone(); // b owns cloned HELLO
    a.push('!'); // a is an owner
    b.push('?');
    //x.push('!'); // x can't modify

    println!("a is {}, b is {}, can't read x", a, b);
    x = a; // x gets "HELLO" back
    println!("can't read a, b is {}, x is {}", b, x);

    drop_string(x); // _s in drop_string is now an owener
    b = return_ownership(b); // b is moved, but got ownership back
    println!("can't read a, b is {}, x is moved", b);

    // borrowing
    let mut x = String::from("HELLO");
    {
        let y = &x; // borrow as immutable
        let z = &x;
        println!("{} {} {}", x, y, z);
    }
    x.push('!'); // y, z have to be destroyed from scope.
    println!("{}", x);

    let y = x.clone();
    let mut z = x.clone();
    let mut zz = x.clone();
    example_fn(x, &y, &mut z, &mut zz);
    //println!("{}", x); // x is moved
    println!("{}", y);
    println!("{}", z);

    // copy by value
    let xx = 42;
    let aa = xx;
    println!("xx {} aa {}", xx, aa);

    // emoji::emoji
    emooji();

    let s = String::from("hello world");

    let hello = &s[0..5]; // borrowed
    let world = &s[6..11]; // borrowed

    let hello2 = &s[..5]; // borrowed
    let world2 = &s[6..]; // borrowed

    // mod::fn
    ownership_in_rust_mod::ownership_in_rust_fn();
    
    ownership_in_rust_folder::ownership_in_rust_mod::ownership_in_rust_fn();

    // file::fn
    reference_in_rust::reference_in_rust();
    reference_in_rust::shared_and_mutable_references();
    reference_in_rust::dereferencing_references();

    // mod.rs
    other_mod::mod_test();
    other_mod::a::a();
    // otehr_mod::b::b(); // b is private


}

fn drop_string(_s: String) {}
fn return_ownership(s: String) -> String {
    s
}

fn example_fn(
    moved: String,
    read_only_borrowed: &String,
    mutable_borrowed: &mut String,
    mutable_ref_borrowed: &mut str,
) {
    println!("moved {}", moved);
    println!("read_only_borrowed {}", read_only_borrowed);
    mutable_borrowed.push('!');
    println!("mutable_borrowed {}", mutable_borrowed);
    let mut x = mutable_ref_borrowed.to_lowercase();
    //mutable_ref_borrowed = x;
    println!("mutable_ref_borrowed {}", mutable_ref_borrowed);
    x.push('.');
    println!("mut x {}", x);
}
