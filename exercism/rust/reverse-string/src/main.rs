mod lib;

use lib::reverse;

fn main() {
  let s = "Karlkim";
  let r = reverse(s);
  println!("{} -> {}", s, r);
}