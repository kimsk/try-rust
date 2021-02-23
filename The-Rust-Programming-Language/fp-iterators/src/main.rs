// OMG ITERATORS https://cglab.ca/~abeinges/talks/iter/#0

// https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
// The iterator returned by into_iter may yield any of T, &T or &mut T, depending on the context.
// The iterator returned by iter will yield &T, by convention.
// The iterator returned by iter_mut will yield &mut T, by convention.

// https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html
// iter() iterates over the items by reference
// into_iter() iterates over the items, moving them into the new scope
// iter_mut() iterates over the items, giving a mutable reference to each item
// So for x in my_vec { ... } is essentially equivalent to my_vec.into_iter().for_each(|x| ... ) 
// - both move the elements of my_vec into the ... scope.

#![allow(unused_variables)]
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {} and {}", val, v1[2]); // can still access vector
    }

    let mut x = 0;
    println!("x: {}", x);
    for i in v1.into_iter() {
        // I got all the data, it's mine!
        x = i;
        println!("{}", x);
    }
    println!("x: {}", x);

    // Oh no! Iterating consumed it :(
    //println!("{}", v1.len()); //~ERROR
}


// Shares the data in the collection
// Read-only access
// Can have many readers at once
#[test]
fn iterator_demonstration() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // need mut
    // calling the next method on an iterator changes internal state
    // The iter method produces an iterator over immutable references.
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    println!("iterator_demonstration: {:?}", v1);
    let x = v1[0];
    println!("iterator_demonstration: {}", x);
    for i in v1 {
        println!("iterator_demonstration: {}", i);
    }
}

#[test]
fn into_iter() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter.
    let mut v1_iter = v1.into_iter();
    assert_eq!(v1_iter.next(), Some(1)); // Option<i32>
    assert_eq!(v1_iter.next(), Some(2));
    assert_eq!(v1_iter.next(), Some(3));
    assert_eq!(v1_iter.next(), None);
    // println!("{:?}", v1); // moved when call into_iter
}

// https://cglab.ca/~abeinges/talks/iter/#5
// IntoIter
// Moves the data out of the collection
// You get total ownership!
// Can do anything with data, including destroy it...

// https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
// if a type implements IntoIterator it can be used in a for loop.
// into_iter is a generic method to obtain an iterator, whether this iterator yields values, immutable references or mutable references is context dependent.

#[test]
fn into_iter_string() {
    let v1: Vec<String> = vec!["1".to_string(), 2.to_string(), 3.to_string()];

    // If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter.
    let mut v1_iter = v1.into_iter();
    assert_eq!(v1_iter.next(), Some("1".to_string())); // Option<String>
    let mut x = v1_iter.next().unwrap(); // Some("2")
    x.push('x');
    println!("into_iter_string: {}", x);
    let mut v1_iter2 = v1_iter;
    assert_eq!(v1_iter2.next(), Some("3".to_string()));
    assert_eq!(v1_iter2.next(), None);
    println!("into_iter_string: {}", x);
    //println!("{:?}", v1); // moved when call into_iter
}

// IterMut - Mutable References (&mut T)
// Loans the data in the collection
// Read-Write access
// Only one loan at once
#[test]
fn iter_mut() {
    let mut data: Vec<String> = vec!["1".to_string(), 2.to_string(), 3.to_string()];
    let iter_mut = data.iter_mut();
    let x: &mut String;
    for string in iter_mut {
        println!("iter_mut: {}", string);
        // mutated!
        string.push_str("!");
        // x = string; // can't share
    }
    // let ccc = iter_mut; // iter_mut has been moved

    let mut iter_mut = data.iter_mut();
    let v = iter_mut.next();
    let s1 = v.unwrap();
    s1.push('#');
    let s2 = s1;
    s2.push('x');

    let mut iter_mut2 = iter_mut;
    let v = iter_mut2.next();
    let s1 = v.unwrap();
    s1.push('#');
    iter_mut2.next();
    let ccc = iter_mut2;



    for string in data.iter() {
        println!("iter_mut - iter: {}", string);
    }

    let mut iter = data.iter();
    let v = iter.next();
    let s1 = v.unwrap();
    //s1.push('$');
    println!("{}", s1);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
    let v1_iter2 = v1_iter;
}

// cargo test -j 1 -- --nocapture