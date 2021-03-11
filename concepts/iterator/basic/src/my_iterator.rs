const MAX: i32 = 5;
pub struct MyIterator {
    // iterator state
    count: i32
}

impl MyIterator {
    pub fn new() -> MyIterator {
        MyIterator{ count: 0 }
    }
}

// impl Iterator Trait for MyIterator
impl Iterator for MyIterator {
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
