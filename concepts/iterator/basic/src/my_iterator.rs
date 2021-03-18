#[derive(Debug)]
pub struct My {
    pub a: String,
    pub b: String,
    c: String,
    something: String
}

impl My {
    pub fn new(a: String, b: String, c: String) -> My {
        My {a, b, c, something: String::from("Something")}
    }

    // iterates over &T
    pub fn iter(&self) -> MyIteratorRef {
        MyIteratorRef::new(self)
    }

    // iterates over &mut T
    pub fn iter_mut(&self) -> MyIteratorRef {
        MyIteratorRef::new(self)
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

// Rust implements IntoIterator for MyIterator for us

// impl IntoIterator for T
// My is consumed by into_iter
impl IntoIterator for My {
    type Item = String;
    type IntoIter = MyIterator;
    fn into_iter(self) -> Self::IntoIter {
        MyIterator::new(self)
    }
}

// impl IntoIterator for &My {
//     type Item = String;
//     type IntoIter = MyIterator;
//     fn into_iter(self) -> Self::IntoIter {
//         MyIterator::new(self)
//     }
// }

#[derive(Debug)]
pub struct MyIteratorRef<'a> {
    position: char,
    my: &'a My
}

impl MyIteratorRef<'_> {
    pub fn new(my: &My) -> MyIteratorRef {
        MyIteratorRef { position: 'a', my }
    }
}

impl<'a> Iterator for MyIteratorRef<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<Self::Item>{
        match self.position {
            'a' => {
                self.position = 'b';
                Some(&self.my.a)
            },
            'b' => {
                self.position = 'c';
                Some(&self.my.b)
            },
            'c' => {
                self.position = 'd';
                Some(&self.my.c)
            },
            _ => None
        }
    }
}

// impl<'a> IntoIterator for My {
//     type Item =  &'a str;
//     type IntoIter = MyIteratorRef<'a>;
//     fn into_iter(self) -> Self::IntoIter {
//         MyIteratorRef::new(&self)
//     }
// }

// impl IntoIterator for &T
impl<'a> IntoIterator for &'a My {
    type Item = &'a str;
    type IntoIter = MyIteratorRef<'a>;
    fn into_iter(self) -> Self::IntoIter {
        MyIteratorRef::new(&self)
    }
}




