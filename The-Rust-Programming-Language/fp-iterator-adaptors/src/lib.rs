#![allow(dead_code)]
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // 1. we call into_iter to create an iterator that takes ownership of the vector.
    shoes.into_iter()
        // 2. Then we call filter to adapt that iterator into a new iterator that only contains elements for which the closure returns true.
        .filter(|s| s.size == shoe_size)
        // 3. calling collect gathers the values returned by the adapted iterator into a vector that’s returned by the function.
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}


#[derive(PartialEq, Debug)]
pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}

#[test]
fn using_other_iterator_trait_methods2() {
    let mut c = Counter::new();
    println!("Counter::new() using `loop`");
    loop {
        match c.next() {
            Some(v) => println!("({})", v),
            _ => break,
        }
    }
    // when using next(), iterator was consumed, and can't be iterated again
    // solution is cloned!
    println!("Counter::new() using `while let`");
    let mut c = Counter::new();
    while let Some(v) = c.next() {
        println!("({})", v);
    }

    let c = Counter::new();
    let c = c.zip(Counter::new().skip(1)).collect::<Vec<_>>();
    println!("zip(Counter::new().skip(1))");
    for &v in &c {
        println!("({}, {})", v.0, v.1);
    }

    let c: Vec<u32> = c.iter().map(|(a, b)| a * b).collect();
    println!("map(|(a, b)| a * b)");
    for &v in &c {
        println!("({})", v);
    }

    let c: Vec<&u32> = c.iter().filter(|x| *x % 3 == 0).collect();
    println!("filter(|x| *x % 3 == 0)");
    for &v in &c {
        println!("({})", v);
    }

    let sum = c.iter().map(|x| *x).sum::<u32>();
    assert_eq!(18, sum);
}


// https://stackoverflow.com/questions/59123462/why-is-iterating-over-a-collection-via-for-loop-considered-a-move-in-rust
// for v in c -- will move iterator
// for &v in &c#[test]
#[test]
fn for_loop_move() {
    let list: Vec<i32> = vec![1, 2, 3, 4];
    for v in list {
        println!("for_loop_move: {}", v);
    }

    // list is moved by for-loop above (into_iter)
    // println!("{:?}", list);
}

#[test]
fn for_loop_borrow() {
    let list: Vec<i32> = vec![1, 2, 3, 4];
    for &v in &list {
        println!("for_loop_borrow: {}", v);
    }

    // list is borrowed by for-loop above
    println!("for_loop_borrow: {:?}", list);
}