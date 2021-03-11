#[derive(Debug)]
pub struct ForwardArrayIterator {
    current_index: usize,
    array: [i32;5]
}

impl ForwardArrayIterator {
    pub fn new(array: [i32;5]) -> ForwardArrayIterator {
        ForwardArrayIterator {
            current_index: 0,
            array
        }
    }
}

impl Iterator for ForwardArrayIterator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.current_index;
        self.current_index += 1;
        if idx >= self.array.len() {
            None
        } else {
            Some (self.array[idx])
        }
    }
}

#[derive(Debug)]
pub struct BackwardArrayIterator {
    current_index: i32,
    array: [i32;5]
}

impl BackwardArrayIterator {
    pub fn new(array: [i32;5]) -> BackwardArrayIterator {
        BackwardArrayIterator {
            current_index: (array.len() as i32) - 1,
            array
        }
    }
}

impl Iterator for BackwardArrayIterator {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        let idx =  self.current_index;
        self.current_index -= 1;
        if idx < 0 {
            None
        } else {
            Some (self.array[idx as usize])
        }
    }
}