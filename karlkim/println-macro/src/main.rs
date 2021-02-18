fn main() {
    print!("สวัสดี");
    let hello = format!("สวัสดี {}", "เจ้า");
    println!("{}", hello);
    println!("{:=^50}", "AMZN");
    println!("{:-<25}+{:->24}", "|", "|");

    #[derive(Debug)]
    struct Person {
        name: &'static str,
        age: u8,
    }

    let p = Person { name: "Rustacean 🦀", age: 42 };

    println!("{:?}", p);
    println!("{:#?}", p);
}

// cargo rustc -- -Z unstable-options --pretty=expanded | rustfmt
// rustup run nightly cargo rustc --profile=check -- -Z unstable-options --pretty expanded
// cargo expand