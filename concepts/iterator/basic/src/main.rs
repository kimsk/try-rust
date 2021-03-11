#![allow(dead_code)]
mod my_iterator;
use my_iterator::MyIterator;

mod array_iterators;

fn run(name: &str, fun: fn() -> ()) {
    println!("{:><50}", name);
    fun();
    println!("{:<>50}", name);
    println!();
}

fn main() {
    println!("Hello, Iterators!");

    run("forward_array_iterator", forward_array_iterator);
    // run("mutated_forward_array_iterato", mutated_forward_array_iterator);
    run("backward_array_iterator", backward_array_iterator);
    // run("my_iterator", my_iterator);
}

fn mutated_forward_array_iterator() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    dbg!(array);
    let forward = array_iterators::ForwardArrayIterator::new(array);
    dbg!(&forward);
    for i in forward {
        // array was cloned to the iterator, so it doesn't affect iterator
        array[1] = 1;
        dbg!(i);
    }
}

fn forward_array_iterator() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    let forward = array_iterators::ForwardArrayIterator::new(array);

    for i in forward {
        dbg!(i);
    }
}

fn backward_array_iterator() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    let backward = array_iterators::BackwardArrayIterator::new(array);

    for i in backward {
        dbg!(i);
    }
}

fn my_iterator() {
    let my_iterator = MyIterator::new();
    for i in my_iterator {
        dbg!(i);
    }
}
