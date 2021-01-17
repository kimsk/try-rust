// https://doc.rust-lang.org/book/ch08-02-strings.html
// New Rustaceans commonly get stuck on strings for a combination of three reasons: Rust’s propensity for exposing possible errors, strings being a more complicated data structure than many programmers give them credit for, and UTF-8.

// It’s useful to discuss strings in the context of collections because strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text.

// Rust has only one string type in the core language, which is the string slice str
// String literals, for example, are stored in the program’s binary and are therefore string slices.
// The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to “strings” in Rust, they usually mean the String and the string slice &str types, not just one of those types.

// A String is a wrapper over a Vec<u8>

// Another point about UTF-8 is that there are actually three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).

// https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str/24159933#24159933

pub fn rust_string() {
  let mut s = String::new();
  s.push('🤪');
  let data = "initial contents";

  let s = data.to_string();

  // the method also works on a literal directly:
  let s = "initial contents".to_string();
  let s = String::from("initial contents");

  hello_utf8();

  let mut s = String::from("foo");
  s.push_str("bar");

  concat_string();

  borrowing();

  access_string();

  bytes_scalar_grapheme("hello");
  bytes_scalar_grapheme("สวัสดี");
  bytes_scalar_grapheme("नमस्ते");

}

fn bytes_scalar_grapheme(hello: &str) {
  println!("bytes_scalar_grapheme(\"{}\")", hello);
  let bytes_len = hello.len();
  println!("hello.len (bytes length) {}", bytes_len);
  for (i, b) in hello.bytes().enumerate() {
    println!("bytes: {} -> {} (&u8)", i, b);
  }

  // Unicode scalar values, which are what Rust’s char type
  // It's important to remember that char represents a Unicode Scalar Value, and may not match your idea of what a 'character' is. Iteration over grapheme clusters may be what you actually want.
  let scalar_len = hello.chars().count();
  println!("hello.len (scalar length) {}", scalar_len);
  for (i, c) in hello.chars().enumerate() {
    println!("scalar: {} -> {} (char)", i, c);
  }
  // grapheme
  use unicode_segmentation::UnicodeSegmentation;
  let grapheme_len = 
    hello
    .graphemes(true)
    .count();

  println!("hello.len (grapheme length) {}", grapheme_len);
  for (i, s) in hello.graphemes(true).enumerate() {
    println!("scalar: {} -> {} (&str)", i, s);
  } 

}

fn borrowing() {
  let b = String::new();
  let mut bb = b; // b is borrowed, and can't be assigned back because it's immutable
  bb.push_str("TEST");
  println!("{}", bb);
  //b = bb; cannot assign twice to immutable variablerustc(E0384)
  let mut bbb = bb; // bb is borrowed
  bbb.push_str(" TEST");
  println!("{}", bbb);
  bb = bbb; // bb is mutable, so it can be assigned back
  println!("{}", bb);
}

fn access_string() {
  let s1 = String::from("hello");
  //let h = s1[0];
  let hello = "Здравствуйте";
  let answer = hello.bytes().nth(0);
}

fn concat_string() {
  let s1 = String::from("Hello, ");
  let mut s2 = String::from("world!");
  let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
  s2.pop(); // remove !
  println!("s3 {} s2 {}", s3, s2);
  //s1 = s3; //cannot assign twice to immutable variablerustc(E0384)
  let mut s1 = String::from("🙏, ");
  s1 += "🌎!";
  let s4 = s1; // s4 borrowed from s1
  println!("s1, s4 {}", s4);
  s1 = s4; // s1 got it back from s4
  s1 += &s2; // s1 did something
  println!("s1 += &s2 {} += {}", s1, s2);
  s2.pop();
  println!("s1 {} s2.len {}", s1, s2.len()); // s2.pop doesn't affect s1
  s2.pop();
  println!("s2 {}", s2);
  s1 += &s2; // s1 did something again
  s2.push_str("- TEST");
  println!("s1 {} s2.len {}", s1, s2.len()); // s2.pop only affects s1
  println!("s2.len {}", s2.len()); // s2.len 
}

fn hello_utf8() {
  let hello = String::from("السلام عليكم");
  let hello = String::from("Dobrý den");
  let hello = String::from("Hello");
  let hello = String::from("שָׁלוֹם");
  let hello = String::from("नमस्ते");
  let hello = String::from("こんにちは");
  let hello = String::from("안녕하세요");
  let hello = String::from("你好");
  let hello = String::from("Olá");
  let hello = String::from("Здравствуйте");
  let hello = String::from("Hola");
  let hello = String::from("สวัสดี 🙏");
  println!("{} 🥑", hello);
}
