#![allow(unused_variables)]

mod string;

fn main() {
    let v = vec![1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    pop112(&mut v);
    println!("{:?}", v);

    // not safe
    // let third: &i32 = &v[5];
    // println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    let first = &v[0];
    v.push(10);
    v.push(20);
    let x = v;
    //println!("{}", first);
    //println!("{:?}", v);

    iterate_immutable();
    iterate_mutable();
    vector_store_enum();

    string::rust_string();
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vector_store_enum() {
    let rows = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for row in rows {
        match row {
            SpreadsheetCell::Int(i) => println!("Int({})", i),
            _ => println!()
        }
    }
}

fn iterate_mutable() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * dereference
    }
    println!("{:?}", v);
}

fn iterate_immutable() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    println!("{:?}", v);
}

fn pop112(v: &mut Vec<i32>) {
    v.push(1);
    v.push(1);
    v.push(2);
}
