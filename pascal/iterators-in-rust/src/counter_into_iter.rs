// http://gradebot.org/doc/ipur/iterator.html
#[derive(PartialEq, Debug)]
pub struct Counter {
    max: i32,
    // `count` tracks the state of this iterator.
    count: i32,
}

impl Counter {
    pub fn new(max: i32) -> Counter {
        Counter { count: -1, max: max }
    }
}

// impl IntoIterator for T
impl IntoIterator for Counter {
    type Item = i32;
    type IntoIter = std::ops::Range<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::ops::Range{ start: 0, end: self.max }
    }
}

// impl IntoIterator for &T
impl<'a> IntoIterator for &'a Counter {
    type Item = i32;
    type IntoIter = std::ops::Range<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        std::ops::Range{ start: 0, end: self.max }
    }
}

pub fn counter_into_iter() {
    dbg!("counter_into_iter::counter_iter");
    let counter = Counter::new(7);
    for i in counter {
        println!("{}", i);
    }
}

pub fn counter_into_iter_reference() {
    dbg!("counter_into_iter::counter_into_iter");
    let counter = Counter::new(7);
    for i in &counter {
        println!("{}", i);
    }

    dbg!(counter);
}
