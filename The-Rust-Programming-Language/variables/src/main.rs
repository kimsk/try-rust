#![allow(unused_variables)]
#![allow(unused_mut)]
const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5_000;
    println!("The value of x is: {}", x);
    x += MAX_POINTS;
    println!("The value of x is: {}", x);

    let mut spaces = "   ";
    let spaces = spaces.len();
    let guess: i128 = "42".parse().expect("Not a number!");
    let xb = 0b111_000_001;
    let xf = 2.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
    let t = true;

    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    let c = 'z';
    let z = 'ℤ';
    let kor_kai = 'ก';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (tx, ty, tz) = tup;

    println!("The value of ty is: {}", ty);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    println!("x.0 is {}", five_hundred);

    let a = [1, 2, 3, 4, 5];
    // let i = 10;
    // let ai = a[i];
    // cargo check is fine
    // cargo build gives error
    println!("a is {:?}", a);
    println!("a[0] is {}, a[1] is {}...", a[0], a[1]);
    let a1 = [3; 5];
    let a2 = [3, 3, 3, 3, 3]; // a1 == a2
    println!("a1 == a2 is {}", a1==a2);

    another_function(a1[4], a1);

    // expression
    let y = {
        let x = 3;
        x + 1 // expression has no semi-colon
    };

    fn fy (x: i32) -> i32 {
        x+100 // no semi-colon
    }

    println!("The value of y is: {}", y);
    let fy12_result = fy(12);
    println!("The value of fy(12) is: {}", fy12_result);

    if_no_else(2);
    if_no_else(6);
    if_else(7);
    if_else_if(27);

    let number = if a1[0] > 3 { 5 } else { 6 };
    println!("if a1[0] > 3 {{ 5 }} else {{ 6 }}; result is {}", number);

    let counter_loop_20_result = counter_loop(20);
    println!("counter_loop_20_result is {}", counter_loop_20_result);

    lift_while(7);
    for_loop(a);
    lift_loop(7);
}

fn for_loop(a: [i32; 5]) {
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn lift_loop(number: i8) {
    for number in (1..number+1).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn lift_while(number: i8) {
    let mut number = number;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn counter_loop(max: i8) -> i16 {
    let mut counter: i16 = 0;

    let result = loop {
        counter += 1;

        if counter == max.into() {
            break counter * 2; // break & return value
        }
    }; // semicolon
    result
}

fn if_no_else(number: i32) {
    println!("if_no_else({})", number);
    if number < 5 {
        println!("condition was true");
    } 
}

fn if_else(number: i32) {
    println!("if_else({})", number);
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_else_if(number: i32) {
    println!("if_else_if({})", number);
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

}


fn another_function(x: i32, a: [i32;5]) {
    println!("Another function. {} {:?}", x, a);
}