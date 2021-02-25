#![allow(dead_code)]
fn main() {
    let x = 10;
    let r = &x;
    println!("r: {}, x: {}", r, x);

    let my_name = "Karlkim SK".to_string();
    let first_name = &my_name[..7];
    println!("my_name: {}, first_name: {}", my_name, first_name);

    let name = "karlkim sk";
    let first_name = &name[..7];
    println!("name: {}, first_name: {}", name, first_name);

    let mut p = Person {
        first_name: "Pascal".to_string(),
        last_name: "Precht".to_string(),
        age: 28
      };
    println!("person.age: {}", p.age);
    mutate_person_age(&mut p, 29);
    println!("person.age: {}", p.age);
    let pp2 = &p;
    println!("person.age: {}", pp2.age);
    let pp = &mut p;
    mutate_person_age(pp, 30);
    println!("person.age: {}", p.age);
}

fn mutate_person_age(p:&mut Person, age: u8) {
    let r = p;
    r.age = age;
}

struct Person {
    first_name: String,
    last_name: String,
    age: u8
}


