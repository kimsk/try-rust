// http://gradebot.org/doc/ipur/iterator.html
#[derive(PartialEq, Debug)]
pub struct Counter2 {
    max: i32,
    // `count` tracks the state of this iterator.
    count: i32,
}

impl Counter2 {
    pub fn new(max: i32) -> Counter2 {
        Counter2 { count: -1, max: max }
    }
}


// http://gradebot.org/doc/ipur/iterator.html#create-iterators
// The interface for iterator, Some(Item) or None (iterations is finished)
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

// The interface for creating an Iterator
// IntoIter is Iterator
// Any type implementing IntoIterator is also called Iterable
// trait IntoIterator where Self::IntoIter::Item == Self::Item {
//     type Item;
//     type IntoIter: Iterator;
//     fn into_iter(self) -> Self::IntoIter;
// }

// It instructs Rust to automatically implement IntoIterator for any type that implements Iterator.
// impl<I> IntoIterator for I where I: Iterator


// impl IntoIterator for T
impl IntoIterator for Counter2 {
    type Item = i32;
    type IntoIter = std::ops::Range<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::ops::Range{ start: 0, end: self.max }
    }
}

// impl IntoIterator for &T
impl<'a> IntoIterator for &'a Counter2 {
    type Item = i32;
    type IntoIter = std::ops::Range<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::ops::Range{ start: 0, end: self.max }
    }
}

// struct Range<Idx> {
//     pub start: Idx,
//     pub end: Idx,
// }

// for i in 0..n { ... }
// is the same as
// for i in std::ops::Range{ start: 0, end: n } { ... }

pub fn counter_into_iter() {
    dbg!("counter_into_iter::counter_iter");
    let counter = Counter2::new(7);
    // impl IntoIterator for T
    for i in counter {
        println!("{}", i);
    }
}

pub fn counter_into_iter_desugar() {
    dbg!("counter_into_iter::counter_into_iter_desugar");
    let counter = Counter2::new(7);
    // impl IntoIterator for T
    let mut iter = IntoIterator::into_iter(counter);
    loop {
        match iter.next() {
            Some(i) => println!("{}", i),
            None => break,
        }
    }
}

pub fn counter_into_iter_reference() {
    dbg!("counter_into_iter::counter_into_iter");
    let counter = Counter2::new(7);
    // impl IntoIterator for &T
    for i in &counter {
        println!("{}", i);
    }

    dbg!(counter);
}
