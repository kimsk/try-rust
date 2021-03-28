#![allow(dead_code, unused_assignments, unused_variables)]

fn main() {
    let mut x = 5;
    {
        let y = &mut x; // &mut T borrow starts here
        *y += 1;
        let z = &x; // &T borrow only possible if the lines below with y is omitted
        // *y += 1;
        // println!("{}", y);
        dbg!(z);
        dbg!(x);
    } // &mut T borrow ends here
    println!("{}", x);

    // iterator_borrow();
    // use_after_free();
}

fn iterator_borrow() {
    let v = vec![1, 2, 3];

    for i in &v {
        println!("{}", i);
    }
}

fn use_after_free() {
    let y: &i32;
    let x = 5;
    y = &x;

    println!("{}", y);
}