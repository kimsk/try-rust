#[derive(PartialEq, Debug)]
pub struct Counter3 {
    max: i32,
    // `count` tracks the state of this iterator.
    count: i32,
}

impl Iterator for Counter3 {
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

impl Counter3 {
    pub fn new(max: i32) -> Counter3 {
        Counter3 { count: -1, max: max }
    }
    
    pub fn iter(&self) -> Counter3 {
        Counter3 { count: -1, max: self.max }
    }
}

pub fn counter3_iter() {
    dbg!("counter_iter::counter3_iter");
    let counter = Counter3::new(7);
    let counter_iter = counter.iter();
    for i in counter_iter {
        println!("{}", i);
    }
    dbg!(counter);
    //dbg!(counter_iter);
}