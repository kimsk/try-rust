#[derive(PartialEq, Debug)]
pub struct Counter3 {
    max: i32,
    // `count` tracks the state of this iterator.
    count: i32,
}

// impl Counter3 {
//     pub fn new(max: i32) -> Counter3 {
//         Counter3 { count: -1, max: max }
//     }

//     pub fn iter() -> &Counter3 {
        
//     }
// }

// pub fn counter_iter() {
//     dbg!("counter_iter::counter_iter");
//     let counter = Counter3::new(7);
//     for i in counter {
//         println!("{}", i);
//     }
//     //dbg!(counter);
// }
