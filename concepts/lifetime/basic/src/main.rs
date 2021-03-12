#![allow(dead_code, unused_variables)]

mod function_lifetime;
mod struct_lifetime;
use struct_lifetime::My;
fn main() {
    function_lifetime::run();

    let s = String::from("Hey");
    let x;
    {
        let my = My { a: 7, b: s.as_str() };
        x = my.b;
        dbg!(my.b());
        dbg!(my.b);
        dbg!(my.bb("Hi"));
    }
    dbg!(x);
    dbg!(s);
}
