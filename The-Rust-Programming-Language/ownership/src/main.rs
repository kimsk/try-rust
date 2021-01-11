fn main() {
    let s = String::from("hellno");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // move occurs
    // s can't be used
    // println!("{}", s);

    let s2 = String::from("hello");
    takes_ownership(s2.clone());
    println!("s2 is cloned! {}", s2);

    let s3 = takes_and_gives_back(s2);
    println!("s2 is back in s3! {}", s3);
    // s3 is ok, but s2 ownership is gone
    // println!("s2 is cloned! {}", s2);

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward
    concepts();
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn concepts() {
  no_double_free();
  no_data_race();
  safe_sharing();
  no_dangling_pointer();

  #[derive(Debug)]
  struct Point {
    x: i32,
    y: i32
  };

  fn no_double_free() {
    println!("no double-free coz one owner");
    let point = Point { x: 0, y: 1 };
    let a = point; // a is the only owner 
    let b = a; // _b is the only owner now
    println!("{:?}", b);
  }

  fn no_data_race() {
    println!("no data race coz one mutable owner");
    let mut point = Point { x: 1, y: 1 };
    println!("{:?}", point);
    point.x = 20;
    let pp = &point; // immutable borrow 
    println!("{:?}", pp);
    point.y = 20; // OK

    let mut a = point; // mutable move
    //point.y = 20; // moved, Not OK
    a.x = 20;
    let mut pp = a;
    pp.y = pp.x;
    println!("{:?}", pp);
  }

  fn safe_sharing() {
    println!("safe sharing coz multiple immutable owners");
    let mut point = Point { x: 2, y: 1 }; 
    point.x = 0;
    let a = &point;
    let b = &point;
    let c = &a;
    println!("{:#?}\na is {:?}, \nb is {:?}, \nc is {:?}", point, a, b, c);
  }

  fn no_dangling_pointer() {
    println!("no dangling-pointer (no use after free) coz borrowed/moved checker");
    let point = Point { x: 2, y: 2 };
    let borrow_before = &point;
    println!("borrow_before is ok {:?}", borrow_before);
    let new_owner = point; 
    println!("new_owner is ok {:?}", new_owner);
    //println!("NOT ok {:?} {:?}", point, borrow_before);
    println!("borrow_before & point are not accessible anymore");
  }
}
