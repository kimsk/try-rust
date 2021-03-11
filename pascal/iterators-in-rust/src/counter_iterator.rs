// http://gradebot.org/doc/ipur/iterator.html
#[derive(PartialEq, Debug)]
pub struct Counter1 {
    max: i32,
    // `count` tracks the state of this iterator.
    count: i32,
}

impl Counter1 {
    pub fn new(max: i32) -> Counter1 {
        Counter1 { count: -1, max: max }
    }
}

// https://doc.rust-lang.org/std/iter/
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }


// impl Iterator for T
impl Iterator for Counter1 {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.max {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn counter_iterator_next() {
    dbg!("counter_iterator::counter_iterator_next");
    let mut counter = Counter1::new(7);
    loop {
        // counter implements Iterator trait, so it has next()
        match counter.next() {
            Some(i) => println!("{}", i),
            None => break,
        }
    }
}

pub fn counter_iterator() {
    dbg!("counter_iterator::counter_iterator");
    let counter = Counter1::new(7);
    // counter implements Iterator trait
    // Rust automatically implements IntoIterator for any type that implements Iterator.
    // impl<I> IntoIterator for I where I: Iterator
    // so it has into_iter and can be consumed in for-loop
    for i in counter {
        println!("{}", i);
    }
    //dbg!(counter);
}
