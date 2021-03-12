pub struct My<'a> {
    pub a: i32,
    pub b: &'a str
}

impl<'a> My<'a> {
    pub fn a(&self) -> i32 {
        self.a
    }

    pub fn b(&self) -> &str {
        self.b
    }

    pub fn bb(&self, str: &'a str) -> &'a str {
        str
    }
}