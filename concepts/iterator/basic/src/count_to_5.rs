const MAX: i32 = 5;

#[derive(Debug)]
pub struct CountTo5 {
    // iterator state
    count: i32
}

impl CountTo5 {
    pub fn new() -> CountTo5 {
        CountTo5{ count: 0 }
    }
}

// impl Iterator Trait for MyIterator
impl Iterator for CountTo5 {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count >= MAX {
            None
        } else {
            self.count +=1;
            Some(self.count)
        }
    }
}
