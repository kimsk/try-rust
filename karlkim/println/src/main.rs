use std::fmt;
use rusty_money::{Money, iso};

#[derive(Debug)]
struct Person {
    name: &'static str,
    age: u8,
}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, age: {}", self.name, self.age)
    }
}

fn main() {
    let p = Person {
        name: "Rustacean 🦀",
        age: 42,
    };

    // positioning
    println!("age: {1}, name: {0}, number: {2}", p.name, p.age, 7);
    // named
    println!("age: {age}, name: {name}, number: {number}",
        name = p.name,
        age = p.age,
        number = 7);

    // fmt::Display
    println!("{}", p);
    // fmt::Debug
    println!("{:?}", p);
    // pretty printing
    println!("{:#?}", p);
    // pointer address of the reference
    println!("{:p}", &p);
    // binary, hexadecimal, octal
    println!("Binary: {:b}, hexadecimal: {:x}, octal: {:o}", p.age, p.age, p.age);
    // unicode as hexadecimal
    println!("{:X}", 'ก' as u32); // E01

    // mix and match
    // pretty print with hexadecimal
    println!("{:#x?}", p);
    // fmt::Debug with named
    println!("{person:?}", person = p);

    // padding
    print_stock("AMZN", 3_290_15, 3_294_95);
    //=======================AMZN=======================
    //|          bid           |          ask          |
    //-------------------------+------------------------
    //|       $3,290.15        |       $3,294.95       |
    //-------------------------+------------------------
}

// padding sample
fn print_stock(ticker: &str, bid: i64, ask: i64) {
    // total length 50, ticker in the middle, pad left and right with =
    println!("{:=^50}", ticker);
    // |
    // total length 24, bid in the middle, pad left and right with space
    // |
    // total length 23, ask in the middle, pad left and right with space
    // |
    println!("|{: ^24}|{: ^23}|", "bid", "ask");
    // - 25, + , - 24 
    println!("{:-^25}+{:-^24}", "", "");

    let bid = Money::from_minor(bid, iso::USD);
    let ask = Money::from_minor(ask, iso::USD);
    println!("|{: ^24}|{: ^23}|", bid.to_string(), ask.to_string());
    println!("{:-^25}+{:-^24}", "", "");
}
