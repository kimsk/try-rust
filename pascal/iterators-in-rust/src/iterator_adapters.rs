// http://gradebot.org/doc/ipur/iterator.html#adapters
// An iterator adapter is a function that takes an iterator and returns another iterator. 
// Common adapters are map(), take(), and filter().
// trait Iterator {
//     type Item;
//     fn filter<P>(self, predicate: P) -> Filter<Self, P> 
//         where P: FnMut(&Self::Item) -> bool;
//     fn map<B, F>(self, f: F) -> Map<Self, F> 
//         where F: FnMut(Self::Item) -> B;
//     fn take(self, n: usize) -> Take<Self>;
// }
pub fn map() {
    dbg!("iterator_adapter::map(|x| x * 2)");
    let range = 0..10;
    let iter = range.map(|x| x * 2);
    for i in iter {
        println!("{}", i);
    }
}

pub fn take() {
    dbg!("iterator_adapter::take(5)");
    let range = 0..10;
    let iter = range.take(5);
    for i in iter {
        println!("{}", i);
    }
}

pub fn filter() {
    dbg!("iterator_adapter::filter(|x| x%2 == 0)");
    let range = 0..10;
    let iter = range.filter(|x| x%2 == 0);
    for i in iter {
        println!("{}", i);
    }
}

pub fn chaining() {
    dbg!("iterator_adapter::chainging");
    let range = 0..10;
    let mapped = range.map(|x| x * 2);
    let taken =  mapped.take(5);
    let filtered = taken.filter(|x| x % 3 == 0);
    let iter = filtered.filter(|x| x%2 == 0);
    for i in iter {
        println!("{}", i);
    }
}