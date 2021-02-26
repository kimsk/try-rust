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
// impl Iterator for T
impl Iterator for Counter {
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

pub fn counter_iter() {
    dbg!("counter_iter::counter_iter");
    let counter = Counter::new(7);
    for i in counter {
        println!("{}", i);
    }
    //dbg!(counter);
}
