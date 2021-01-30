#![allow(dead_code)]
use std::fmt;
use rusty_money::{Money, iso};
use colored::*;

#[derive(Debug)]
enum Team {
    Red,
    Green,
    Blue
}
impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Team::Red => write!(f, "🔴"),
            Team::Green => write!(f, "🟢"),
            Team::Blue => write!(f, "🔵"),
        }
    }
}

#[derive(Debug)]
struct Person {
    name: &'static str,
    age: u8,
    team: Team
}
impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "team: {}, name: {}, age: {}", self.team, self.name, self.age)
    }
}

fn main() {
    // let p = Person { name: "Rustacean 🦀", age: 42, team: Team::Red };
    // // positioning
    // println!("team: {2}, name: {0}, age: {1}", p.name, p.age, p.team);
    // // named
    // println!("team: {team}, name: {name}; {motto}",
    //     name = p.name,
    //     team = p.team,
    //     motto = "Come on you Reds!");

    // let p2 = Person { name: "SeeSharp", age: 60, team: Team::Blue };
    // println!("{rust:#?} พบกับ {csharp:#?}", rust = p, csharp = p2);

    // // fmt::Display
    // println!("{}", p);
    // // fmt::Debug
    // println!("{:?}", p);
    // // pretty printing
    // println!("{:#?}", p);
    // // pointer address of the reference
    // println!("{:p}", &p);
    // // binary, hexadecimal, octal
    // println!("Binary: {:b}, hexadecimal: {:x}, octal: {:o}", p.age, p.age, p.age);
    // // unicode as hexadecimal
    // println!("{:X}", 'ก' as u32); // E01

    // // mix and match
    // // pretty print with hexadecimal
    // println!("{:#x?}", p);
    // // fmt::Debug with named
    // println!("{person:?}", person = p);

    // // padding
    // let stocks = vec![
    //     ("AMZN", 3_290_15, 3_294_95),
    //     ("GOOGL", 1885_00, 1902_09),
    //     ("FB", 275_70, 276_20) 
    // ];

    // for (ticker, bid, ask) in stocks {
    //     print_stock(ticker, bid, ask);
    // }
// =======================AMZN=======================
// |          bid           |          ask          |
// -------------------------+------------------------
// |       $3,290.15        |       $3,294.95       |
// -------------------------+------------------------
// ======================GOOGL=======================
// |          bid           |          ask          |
// -------------------------+------------------------
// |       $1,885.00        |       $1,902.09       |
// -------------------------+------------------------
// ========================FB========================
// |          bid           |          ask          |
// -------------------------+------------------------
// |        $275.70         |        $276.20        |
// -------------------------+------------------------

    //print_padding();
    let stocks = vec![
        ("AMZN", 3_290_15, 3_294_95),
        ("GOOGL", 1885_00, 1902_09),
        ("FB", 275_70, 276_20),
        ("GME", 312_00, 315_00)
    ];

    for (ticker, bid, ask) in stocks {
        print_stock(ticker, bid, ask);
    }

    //
    // println!("Hello {1} is {2:.0$}", 5, "x", 5100.26);
    // println!("Hello {} is {:.*}", "x", 5, 0.01);

}

// padding sample
fn old_print_stock(ticker: &str, bid: i64, ask: i64) {
    // total length 50, ticker in the middle, pad left and right with =
    println!("{:=^50}", ticker.blue().bold());
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

//0111111111111111111100AMZN00111111111111111111110

fn print_stock(ticker: &str, bid: i64, ask: i64) {
    let stock = format!(" {} ", ticker).bold();
    let table_width = 50;
    let ticker_width = stock.len();
    let left_width = (table_width - ticker_width)/2;
    let right_width = (table_width - ticker_width)/2;
    let bid = Money::from_minor(bid, iso::USD);
    let ask = Money::from_minor(ask, iso::USD);

    println!("table_length: {}", table_width);
    println!("ticker_width: {}", ticker_width);
    println!("left_width: {}", left_width);
    println!("right_width: {}", right_width);

    println!("{:>left_width$}{:─^ticker_width$}{:<right_width$}", "┌", "", "┐", left_width=left_width, right_width=right_width, ticker_width=ticker_width);
    println!("┌{:─>left_width$}┤{}├{:─>right_width$}┐", "", stock, "", left_width=left_width-2, right_width=right_width-2);
    println!("│{: ^left_width$}└{:─^ticker_width$}┘{: ^right_width$}│", "bid", "┬", "ask", left_width=left_width-2, right_width=right_width-2, ticker_width=ticker_width);
    println!("├{:─^left_width$}{:─^ticker_width$}{:─^right_width$}┤", "", "┼", "", left_width=left_width-1, right_width=right_width-1, ticker_width=ticker_width);
    println!("│{: ^left_width$}{: ^ticker_width$}{: ^right_width$}│", bid.to_string(), "│", ask.to_string(), left_width=left_width-1, right_width=right_width-1, ticker_width=ticker_width);
    println!("└{:─^left_width$}{:─^ticker_width$}{:─^right_width$}┘", "", "┴", "", left_width=left_width-1, right_width=right_width-1, ticker_width=ticker_width);
}


// padding
fn print_padding() {
    // stock variable
    // :
    // padding character: =
    // padding alignment: ^ (middle)
    // padding size: 50
    println!("{stock:=^50}", stock = "GME".blue().bold());

    println!("{stock:=<50}", stock = "GME".blue().bold());

    println!("{stock:=>50}", stock = "GME".blue().bold());

    // "|"
    // " " padding right < until 25
    println!("{: <25}|{: >24}", "|", "|");
    println!("|{: ^24}|{: ^23}|", "bid", "ask");

    println!("{:-^25}+{:-^24}", "", "");

    // new line
    println!("{:\n^5}", "")
}