// cargo test --test integration_test

// https://doc.rust-lang.org/edition-guide/rust-2018/module-system/path-clarity.html
//extern crate integration_tests as adder; // Rust 2015
//use ::integration_tests as adder;
use integration_tests as adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}