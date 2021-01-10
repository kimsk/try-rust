#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

// https://blog.thoughtram.io/references-in-rust/

// References enable us to give things like functions an data structures access to values,
// without transferring ownership

// borrow-operator &

pub fn reference_in_rust() {
  let x = 10;
  let r = &x;

  let my_name = "Pascal Precht".to_string();
  let last_name = &my_name[7..];
  let last_name = last_name.to_string();
  println!("my_name.capacity {}", my_name.capacity());
  println!("last_name.capacity {}", last_name.capacity());
}

pub fn shared_and_mutable_references() {
  struct Person {
    first_name: String,
    last_name: String,
    age: u8,
  }
  let mut p = Person {
    first_name: "Pascal".to_string(),
    last_name: "Precht".to_string(),
    age: 28,
  };
  let r = &mut p;
  println!("r.age {}", r.age);
  r.age = 29;
  println!("r.age {}", r.age);
  let mut r2 = &mut p;
  r2.age = 30; 
  println!("r2.age {}", r2.age);
  let r3 = &p;
  println!("r3.age {}", r3.age);
  let r4 = &&r3;
  println!("r4.age {}", r4.age);

}

pub fn dereferencing_references() {
  let x = 10;
  let r = &&x;

  if **r == 10 {
    println!("Same! {} {}", x, r); //
  }
}