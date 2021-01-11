#![allow(unused_variables)]
#![allow(dead_code)]

// https://tourofrust.com/chapter_1_en.html
fn main() {

    let x;
    x = 0; // i32 by default
    let z = &x + 10;
    let c = 4.3f32;
    let s = "test";
    let b = c as i32 + z; // casting
    let t = true;
    println!("{}", t as u8);
    const HELLO_WORLD: &str = "Hello World!";
    let hello_world = String::from("Hello World!");
    println!("{} {}", HELLO_WORLD, hello_world);
    let big = 10_000_000_000_000i64;
    println!("i64 {}\ni128 {}\ni32 {}\nu16 {}", big, big as i128, big as i32, big as u16);
    
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);

    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}
// https://tourofrust.com/12_en.html