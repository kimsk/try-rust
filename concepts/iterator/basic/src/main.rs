#![allow(dead_code)]
mod count_to_5;
use array_iterators::BackwardArrayIterator;
use count_to_5::CountTo5;

mod my_iterator;
use my_iterator::{My, MyIterator, MyIteratorRef};

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
    // run("backward_array_iterator_for_loop", backward_array_iterator_for_loop);
    // run("backward_array_iterator", backward_array_iterator);
    // run("count_to_5_for_loop", count_to_5_for_loop);
    // run("count_to_5_iterator", count_to_5_iterator);

    // run("my_iterator", my_iterator);
    // run("my_iterator_next", my_iterator_next);
    // run("my_iterator_into_iter", my_iterator_into_iter);
    // run("my_iterator_into_iter2", my_iterator_into_iter2);
    // run("my_iterator_into_iter3", my_iterator_into_iter3);
    // run("my_iterator_into_iter4", my_iterator_into_iter4);
    // run("my_iterator_into_iter_for_each", my_iterator_into_iter_for_each);
    // run("my_iterator_into_iter_filter", my_iterator_into_iter_filter);

    // run("my_iterator_ref", my_iterator_ref);
    // run("my_iterator_ref_borrowed", my_iterator_ref_borrowed);
    // run("my_iterator_ref_borrowed_mut_ref", my_iterator_ref_borrowed_mut_ref);
    // run("my.iter()", my_iter);
    // run("my.into_iter()", my_into_iter);

    let s3 = "s3";
    let s4 = String::from("s4");
    let s = one_collection_two_iterators(s3, &s4);
    dbg!(s);
    dbg!(s3);
    dbg!(s4);
}

fn one_collection_two_iterators<'a>(s3: &'a str, s4: &'a String) -> (String, &'a String) {
    let s1 = String::from("s1");
    let s2 = String::from("s2");
    let s3 = String::from(s3);
    let v = vec![&s1, &s2, &s3, &s4];
    dbg!(&s1);
    let _v0 = v[0];
    let v2 = vec![&s2, &s3, &s1, &s4];
    dbg!(v);
    dbg!(v2);
    (s1, s4)
}

fn my_iter() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    for i in my.iter() {
        dbg!(i);
    }
    dbg!(my);
}

fn my_into_iter() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    for i in my.into_iter() {
        dbg!(i);
    }
    //dbg!(my);
}


fn my_iterator_ref_borrowed_mut_ref() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    let mut iter = MyIteratorRef::new(&my);
    for i in &mut iter {
        dbg!(i);
    }
    dbg!(iter);
}

fn my_iterator_ref_borrowed_ref() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    for i in &my {
        dbg!(i);
    }
    dbg!(my);
}

fn my_iterator_ref_borrowed() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    let iter = MyIteratorRef::new(&my);
    for i in iter {
        dbg!(i);
    }
    //dbg!(my); // my is borrowed
    // dbg!(iter); // iter is moved when calling for loop
}

fn my_iterator_ref() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    let mut iter = MyIteratorRef::new(&my);
    dbg!(&my.a);
    let x = iter.next();
    dbg!(x);
    dbg!(my); // my is not consumed
}


fn my_iterator_next() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    let mut iter = my.into_iter();
    dbg!(iter.next());
}

fn my_iterator_into_iter_filter() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    let iter = My::into_iter(my).filter(|c| c == "A");
    for c in iter {
        dbg!(c);
    }
}

fn my_iterator_into_iter_for_each() {
    let my = My::new(String::from("A"), String::from("B"), String::from("C"));
    let iter = IntoIterator::into_iter(my);
    iter.for_each(|c| {
        dbg!(c);
    });
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
    let _a = &my.a;
    dbg!(_a);
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
    // dbg!(my); // my is consumed by My::into_iter
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
    // dbg!(my); // my is consumed by MyIterator::new
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

    let mut backward = array_iterators::BackwardArrayIterator::new(array);
    dbg!(BackwardArrayIterator::next(&mut backward));
    dbg!(backward.next());
    dbg!(backward.next());
    dbg!(backward.next());
    dbg!(backward.next());
    dbg!(BackwardArrayIterator::next(&mut backward));
}

fn backward_array_iterator_for_loop() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    let backward = array_iterators::BackwardArrayIterator::new(array);

    for i in backward {
        dbg!(i);
    }
}

fn count_to_5_iterator() {
    let mut count_to_5 = CountTo5::new();
    dbg!(count_to_5.next());
    dbg!(count_to_5.next());
    dbg!(count_to_5.next());
    dbg!(count_to_5.next());
    dbg!(count_to_5.next());
    dbg!(count_to_5.next());
}

fn count_to_5_for_loop() {
    let count_to_5 = CountTo5::new();
    for i in count_to_5 {
        dbg!(i);
    }
}
