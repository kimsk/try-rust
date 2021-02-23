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

// ANCHOR: here
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
// ANCHOR_END: here

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
    let c = Counter::new();
    println!("counter: {:#?}", c);
    let c = c.zip(Counter::new().skip(1));
    println!("zip: {:#?}", c);
    let c = c.map(|(a, b)| a * b);
    println!("map: {:#?}", c);
    let c = c.filter(|x| x % 3 == 0);
    println!("filter: {:#?}", c);
    let sum = c.sum::<u32>();
    assert_eq!(18, sum);
}