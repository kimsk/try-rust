// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#the-borrow-checker
// 5 is allocated in stack, owned by x, dropped at the end of scope, so r can't have a reference to it
// fn borrow_checker() {
//     {
//         let r;
//         {
//             let x = 5;
//             r = &x;
//         }
//         println!("r: {}", r);
//     }
// }

pub fn run() {
    let z;
    let x = String::from("X");
    {
        let y = String::from("Y"); // Not OK, "Y" in heap and owned by y
        let y = "Y"; // OK because "Y" (string literal) has 'static lifetime and y only borrows "Y"
        // https://stackoverflow.com/questions/57225055/in-rust-can-you-own-a-string-literal

        z = longest0(&x, &y);
        println!("{} {}", x, y);
    }
    println!("{}", z);
}

// 'a lifetime for both x & y as both can be returned
fn longest0<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// y doesn't need lifetime as it's not returned.
fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    println!("{}", y);
    x
}

// no lifetime needed for parameters
// return String (not a reference)
fn longest2(x: &str, y: &str) -> String {
    println!("{}, {}", x, y);
    let result = String::from("really long string");
    result
}

// Lifetime Elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}