fn main() {
    let s = format!("{:-<5}", 11);
    print!("{:#^5}", 12);
    println!("{:*>5}", 13);

}

fn borrow_thing(s: &str) -> () {
    println!("{}", s);
}

fn borrow_thing2<'a>(s: &'a str) -> &'a str {
    println!("{}", s);
    s
}


// https://www.oreilly.com/library/view/rust-programming-by/9781788390637/e07dc768-de29-482e-804b-0274b4bef418.xhtml
// rustup default nightly
// rustc test_println.rs -Z unstable-options --pretty expanded
// rustup default stable
#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std;
fn main() {
    let s =




        // https://www.oreilly.com/library/view/rust-programming-by/9781788390637/e07dc768-de29-482e-804b-0274b4bef418.xhtml
        // rustup default nightly
        // rustc test_println.rs -Z unstable-options --pretty expanded
        // rustup default stable
        {
            let res =
                ::alloc::fmt::format(::core::fmt::Arguments::new_v1_formatted(&[""],
                                                                              &match (&11,)
                                                                                   {
                                                                                   (arg0,)
                                                                                   =>
                                                                                   [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                 ::core::fmt::Display::fmt)],
                                                                               },
                                                                              &[::core::fmt::rt::v1::Argument{position:
                                                                                                                  0usize,
                                                                                                              format:
                                                                                                                  ::core::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                                      '-',
                                                                                                                                                  align:
                                                                                                                                                      ::core::fmt::rt::v1::Alignment::Left,
                                                                                                                                                  flags:
                                                                                                                                                      0u32,
                                                                                                                                                  precision:
                                                                                                                                                      ::core::fmt::rt::v1::Count::Implied,
                                                                                                                                                  width:
                                                                                                                                                      ::core::fmt::rt::v1::Count::Is(5usize),},}]));
            res
        };
    ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(&[""],
                                                               &match (&12,) {
                                                                    (arg0,) =>
                                                                    [::core::fmt::ArgumentV1::new(arg0,
                                                                                                  ::core::fmt::Display::fmt)],
                                                                },
                                                               &[::core::fmt::rt::v1::Argument{position:
                                                                                                   0usize,
                                                                                               format:
                                                                                                   ::core::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                       '#',
                                                                                                                                   align:
                                                                                                                                       ::core::fmt::rt::v1::Alignment::Center,
                                                                                                                                   flags:
                                                                                                                                       0u32,
                                                                                                                                   precision:
                                                                                                                                       ::core::fmt::rt::v1::Count::Implied,
                                                                                                                                   width:
                                                                                                                                       ::core::fmt::rt::v1::Count::Is(5usize),},}]));
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(&["",
                                                                     "\n"],
                                                                   &match (&13,)
                                                                        {
                                                                        (arg0,)
                                                                        =>
                                                                        [::core::fmt::ArgumentV1::new(arg0,
                                                                                                      ::core::fmt::Display::fmt)],
                                                                    },
                                                                   &[::core::fmt::rt::v1::Argument{position:
                                                                                                       0usize,
                                                                                                   format:
                                                                                                       ::core::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                           '*',
                                                                                                                                       align:
                                                                                                                                           ::core::fmt::rt::v1::Alignment::Right,
                                                                                                                                       flags:
                                                                                                                                           0u32,
                                                                                                                                       precision:
                                                                                                                                           ::core::fmt::rt::v1::Count::Implied,
                                                                                                                                       width:
                                                                                                                                           ::core::fmt::rt::v1::Count::Is(5usize),},}]));
    };
}
fn borrow_thing(s: &str) -> () {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                         &match (&s,) {
                                                              (arg0,) =>
                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                            ::core::fmt::Display::fmt)],
                                                          }));
    };
}
fn borrow_thing2<'a>(s: &'a str) -> &'a str {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(&["", "\n"],
                                                         &match (&s,) {
                                                              (arg0,) =>
                                                              [::core::fmt::ArgumentV1::new(arg0,
                                                                                            ::core::fmt::Display::fmt)],
                                                          }));
    };
    s
}