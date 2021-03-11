#[derive(Debug)]
pub struct My {
    a: String,
    b: String,
    c: String,
    something: String
}

impl My {
    pub fn new(a: String, b: String, c: String) -> My {
        My {a, b, c, something: String::from("Something")}
    }
}

pub struct MyIterator {
    position: char,
    my: My
}

impl MyIterator {
    pub fn new(my: My) -> MyIterator {
        MyIterator { position: 'a', my }
    }
}

impl Iterator for MyIterator {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item>{
        match self.position {
            'a' => {
                self.position = 'b';
                Some(self.my.a.clone())
            },
            'b' => {
                self.position = 'c';
                Some(self.my.b.clone())
            },
            'c' => {
                self.position = 'd';
                Some(self.my.c.clone())
            },
            _ => None
        }
    }
}

// My is consumed by into_iter
impl IntoIterator for My {
    type Item = String;
    type IntoIter = MyIterator;
    fn into_iter(self) -> Self::IntoIter {
        MyIterator::new(self)
    }
}
