#![allow(dead_code)]

#[derive(Debug)] // let you print {:?}
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

fn main() {
    let mut pascal = Person {
        first_name: "Pascal".to_string(),
        last_name: "Precht".to_string(),
        age: 28,
    };
    let r = &pascal;
    println!("Hello, {}!", r.first_name);
    // `.` operator performs the dereferencing implicitly
    println!("Hello, {}!", (*r).first_name);

    let r = &pascal; // borrow as immutable
    borrow_person(r);
    println!("pascal not changed {:#?}", pascal);
    borrow_and_mutate_person(&mut pascal);
    println!("pascal.age changed from 28 -> {}", pascal.age);
    move_person(pascal);

    let karlkim = Person {
        first_name: String::from("Karlkim"),
        last_name: String::from("SK"),
        age: 45 
    };

    borrow_person(&karlkim);
    // borrow_and_mutate_person(&mut karlkim);
    // cannot borrow `karlkim` as mutable, as it is not declared as mutable
    move_person(karlkim);
    // borrow_person(&karlkim);
    // borrow of moved value: `karlkim`

    let mut mutable_numbers: [i32;3] = [3, 1, 2];
    println!("original mutable_numbers {:?}", mutable_numbers);
    mutable_numbers.sort(); // sort(&mut self) -- mutable borrow
    // `.` operator will implicitly borrow a reference to its left operand
    println!("sorted mutable_numbers {:?}", mutable_numbers);
    sort_desc(&mut mutable_numbers);
    println!("sorted desc mutable_numbers {:?}", mutable_numbers);
}

fn sort_desc(numbers: &mut [i32;3]) {
    numbers.sort_by(|a, b| b.cmp(a));
}

fn move_person(_p: Person) {}
fn borrow_person(_p: &Person) {}
fn borrow_and_mutate_person(p: &mut Person) {
    p.age = 30;
}
