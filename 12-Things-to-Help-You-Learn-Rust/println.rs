fn main() {
  println!("Hello");
  let x = 5;
  let y = 6;
  println!("x is {}, y is {}", x, y);
  println!("x is {x}, y is {y}, z is {z}", x=x, y=y, z=x+y);
  let tup = (x,y,x+y);
  println!("(x,y,z) is {:?}", tup);
  let (_, _, z) = tup;
  println!("y is {1}, x is {0}, z is {2}", x, y, z);
}

// rustc println.rs