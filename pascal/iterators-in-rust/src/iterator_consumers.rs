// http://gradebot.org/doc/ipur/iterator.html#consumers
// A consumer also takes an iterator, but instead of returning another iterator,
// it returns a value that is not an iterator.
// Common consumers are find(), collect(), and fold().

// trait Iterator {
//     fn collect<B>(self) -> B where B: FromIterator<Self::Item>;
//     fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where P: FnMut(&Self::Item) -> bool;
//     fn fold<B, F>(self, init: B, f: F) -> B where F: FnMut(B, Self::Item) -> B;
// }

// trait FromIterator<A> {
//     fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=A>;
// }

pub fn find() {
    dbg!("iterator_consumers::find(|x| *x == 10");
    let mut r = 0..100;
    let ten = r.find(|x| *x == 10);

    dbg!(ten);
}

pub fn collect_i32() {
    dbg!("iterator_consumers::collect::<Vec<i32>>");
    let r = 0..5;
    // Vec<i32> implements FromIterator<i32>
    let collected = r.collect::<Vec<i32>>();
    dbg!(collected);
}

#[derive(Debug)]
struct Foo<T> {
    v: Vec<T>,
}

impl<A> std::iter::FromIterator<A> for Foo<A> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item=A> {
        let mut foo = Foo{ v: vec![] };
        for i in iter {
            foo.v.push(i);
        }
        foo
    }
}

pub fn collect_foo_i32() {
    dbg!("iterator_consumers::collect::<<Foo<i32>>");

    let r = 0..5;
    // Vec<i32> implements FromIterator<i32>
    let collected = r.collect::<Foo<i32>>();
    let collected = dbg!(collected);
    for v in collected.v {
        println!("{}", v);
    }
}

pub fn fold() {
    dbg!("iterator_consumers::fold()");
    let r = 1..5;
    let sqr = |x| x * x;
    let add = |x, y| x + y;
    let squared = r.map(sqr);
    let added = squared.fold(0, add) ;
    dbg!(added);
}
