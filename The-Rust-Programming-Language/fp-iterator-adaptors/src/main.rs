use fp_iterator_adaptors::Counter;

// https://doc.rust-lang.org/book/ch13-02-iterators.html#methods-that-produce-other-iterators
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v1_mapped = v1.iter().map(|x| x + 1);
    for v in v1_mapped {
        println!("{}", v);
    }

    let c = Counter::new();
    for v in c {
        println!("counter: {}", v);
    }
}

#[test]
fn lazy() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v1_mapped = v1.iter().map(|x| x + 1);
    for v in v1_mapped {
        println!("lazy: {}", v);
    }
}

#[test]
fn consume_iter() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v1_mapped = v1.iter().map(|x| x + 1);
    for v in v1_mapped {
        println!("consume_iter: {}", v);
    }
}
#[test]
fn collect_iter() {
    use std::iter::Map;
    use std::slice::Iter;

    let y = 1;
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Map<Iter<i32>, _> = v1.iter().map(|x| x + y); // lazy
    for v in v2 {
        println!("collect_iter: {}", v);
    }
    // let v3: Vec<_> = v2.collect(); // collect

    // assert_eq!(v3, vec![2, 3, 4]);
}