#![allow(dead_code)]
mod count_to_5;
use count_to_5::CountTo5;

mod my_iterator;
use my_iterator::{My, MyIterator};

mod array_iterators;

fn run(name: &str, fun: fn() -> ()) {
    println!("{:><50}", name);
    fun();
    println!("{:<>50}", name);
    println!();
}

fn main() {
    println!("Hello, Iterators!");

    // run("forward_array_iterator", forward_array_iterator);
    // run("mutated_forward_array_iterato", mutated_forward_array_iterator);
    // run("backward_array_iterator", backward_array_iterator);
    // run("count_to_5", count_to_5);

    run("my_iterator", my_iterator);
    run("my_iterator_into_iter", my_iterator_into_iter);
    run("my_iterator_into_iter2", my_iterator_into_iter2);
    run("my_iterator_into_iter3", my_iterator_into_iter3);
    run("my_iterator_into_iter4", my_iterator_into_iter4);
    run("my_iterator_into_iter_filter", my_iterator_into_iter_filter);
}

fn my_iterator_into_iter_filter() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    let iter = My::into_iter(my).filter(|c| c == "A");
    for c in iter {
        dbg!(c);
    }
}

fn my_iterator_into_iter4() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    let iter = IntoIterator::into_iter(my);
    for c in iter {
        dbg!(c);
    }
}

fn my_iterator_into_iter3() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    let iter = My::into_iter(my);
    // dbg!(my);// my is consumed by into_iter
    for c in iter {
        dbg!(c);
    }
}

fn my_iterator_into_iter2() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    // let iter = my.into_iter(); // OO version
    let iter = My::into_iter(my); // non OO version
    for c in iter {
        dbg!(c);
    }
}

fn my_iterator_into_iter() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    for c in my {
        dbg!(c);
    }
}

fn my_iterator() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    let my_iterator = MyIterator::new(my);
    for c in my_iterator {
        dbg!(c);
    }
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

fn count_to_5() {
    let count_to_5 = CountTo5::new();
    for i in count_to_5 {
        dbg!(i);
    }
}
