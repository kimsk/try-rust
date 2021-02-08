# [The Rust Programming Language](https://doc.rust-lang.org/book/)

## Chapter 1: Getting Started
## Chapter 2: Programming a Guessing Game
## Chapter 3: Common Programming Concepts
## Chapter 4: Understanding Ownership
> All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that constantly looks for no longer used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at compile time. None of the ownership features slow down your program while it’s running.

## Chapter 5: Using Structs to Structure Related Data
1. **Structs** let you create custom types that are meaningful for your domain.
2. **Methods** let you specify the behavior that instances of your structs have, and associated functions let you namespace functionality that is particular to your struct without having an instance available.

## Chapter 6: Enums and Pattern Matching
> Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.
1. Use **enum** to create customer types that can be one of a set of enumerated values.
2. When **enum** values have data inside them, you can use `match` or `if` `let` to extract and use those values

## Chapter 7: Managing Growing Projects with Packages, Crates, and Modules
1. Rust lets you split a **package** into multiple **crates** and a **crate** into **modules**
2. Module code is **private** by default, but you can make definitions public by adding the `pub` keyword.
3. Module paths can be brought into scope with a `use` statement.

## Chapter 8: Common Collections
> Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data.

## Chapter 9: Error Handling
1. The `panic!` macro signals that your program is in a state it can’t handle and lets you tell the process to stop instead of trying to proceed with invalid or incorrect values.
2. The `Result` enum uses Rust’s type system to indicate that operations might fail in a way that your code could recover from. You can use `Result` to tell code that calls your code that it needs to handle potential success or failure as well.


## Chapter 10: Generic Types, Traits, and Lifetimes
1. Generic type parameters let you apply the code to different types.
2. Traits and trait bounds ensure that even though the types are generic, they’ll have the behavior the code needs.
3. You learned how to use lifetime annotations to ensure that this flexible code won’t have any **dangling references**.

## Chapter 11: Writing Automated Tests
1. The convention for **unit tests** is to create a module named tests in each file to contain the test functions and to annotate the module with `cfg(test)`.
1. The `#[cfg(test)]` annotation on the tests module tells Rust to compile and run the test code only when you run `cargo test`, not when you run `cargo build`.
1. In Rust, integration tests are entirely external to your library. They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API.