#![allow(dead_code)]
use std::fmt;
use std::env;

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
    let args: Vec<String> = env::args().collect();
    let table_width =  if args.len() > 1 {
        match args[1].parse() {
        Ok(v) => v,
        _ => 50,
    }} else { 50 };
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

    // capturing
    let print_stock_pretty = |ticker: &str, bid: i64, ask: i64| {
        print_stock_pretty_with_table_width(table_width, ticker, bid, ask);
    };

    //print_padding();
    let stocks = vec![
        ("AMZN", 3_290_15, 3_294_95),
        ("GOOGL", 1885_00, 1902_09),
        ("FB", 275_70, 276_20),
        ("GME", 312_00, 315_00),
    ];

    for (ticker, bid, ask) in stocks {
        // print_stock_pretty(ticker, bid, ask);
        //print_stock_pretty_with_table_width(table_width, ticker, bid, ask);
        print_stock(ticker, bid, ask);
        print_stock_pretty(ticker, bid, ask);
    }

    //
    // println!("Hello {1} is {2:.0$}", 5, "x", 5100.26);
    // println!("Hello {} is {:.*}", "x", 5, 0.01);

}

// padding sample
// =======================AMZN=======================
// |          bid           |          ask          |
// |------------------------+-----------------------|
// |       $3,290.15        |       $3,294.95       |
// -------------------------+------------------------
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
    println!("{:-<25}+{:->24}", "|", "|");

    let bid = Money::from_minor(bid, iso::USD);
    let ask = Money::from_minor(ask, iso::USD);
    println!("|{: ^24}|{: ^23}|", bid.to_string(), ask.to_string());
    println!("{:->25}+{:-<24}", "", "");
}

// pretty
//                       ┌────┐                      
// ┌─────────────────────┤AMZN├─────────────────────┐
// │         bid         └─┬──┘         ask         │
// ├───────────────────────┼────────────────────────┤
// │      $3,290.15        │        $3,294.95       │
// └───────────────────────┴────────────────────────┘
fn print_stock_pretty(ticker: &str, bid: i64, ask: i64) {
    print_stock_pretty_with_table_width(50, ticker, bid, ask);
}

fn print_stock_pretty_with_table_width(table_width: usize, ticker: &str, bid: i64, ask: i64) {
    let ticker_width = ticker.len();
    let left_width = (table_width - ticker_width)/2;
    let right_width = if ticker_width %2 == 0 {
        (table_width - ticker_width)/2
    } else {
        (table_width - ticker_width)/2 + 1 
    };
    let bid = Money::from_minor(bid, iso::USD);
    let ask = Money::from_minor(ask, iso::USD);

    // println!("table_length: {}", table_width);
    // println!("ticker_width: {}", ticker_width);
    // println!("left_width: {}", left_width);
    // println!("right_width: {}", right_width);

    println!("{:>left_width$}{:─^ticker_width$}{:<right_width$}", "┌", "", "┐", left_width=left_width, right_width=right_width, ticker_width=ticker_width);
    println!("┌{:─>left_width$}┤{}├{:─>right_width$}┐", "", ticker.blue(), "", left_width=left_width-2, right_width=right_width-2);
    println!("│{: ^left_width$}└{:─^ticker_width$}┘{: ^right_width$}│", "bid".green(), "┬", "ask".red(), left_width=left_width-2, right_width=right_width-2, ticker_width=ticker_width);
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