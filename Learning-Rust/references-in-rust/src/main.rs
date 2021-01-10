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

    let mut mutable_numbers: [i32;3] = [3, 1, 2];
    mutable_numbers.sort(); // sort(&mut self) -- mutable borrow
    println!("mutable_numbers {:?}", mutable_numbers);

    let r = &pascal; // borrow as immutable
    borrow_person(r);
    println!("pascal not changed {:#?}", pascal);
    borrow_and_mutate_person(&mut pascal);
    println!("pascal.age changed from 28 -> {}", pascal.age);
    move_person(pascal);

}

fn move_person(_p: Person) {}
fn borrow_person(_p: &Person) {}
fn borrow_and_mutate_person(p: &mut Person) {
    p.age = 30;
}
