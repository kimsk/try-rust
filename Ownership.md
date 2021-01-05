## [What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
> Some languages (e.g., C#, Java) have garbage collection that constantly looks for no longer used memory as the program runs; in other languages (e.g., C, C++), the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks at **compile time**. 

> Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses. 

## Ownership Rules:
- Each value in Rust has a variable that’s called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

> In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII).

> The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

## Borrowing
> Taking ownership and then returning ownership with every function is a bit tedious. What if we want to let a function use a value but not take ownership? It’s quite annoying that anything we pass in also needs to be passed back if we want to use it again, in addition to any data resulting from the body of the function that we might want to return as well.

## Immutable/Mutable References & Data Race

### Data Race
A data race is similar to a race condition and happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

### References

- Only one mutable reference to a particular piece of data in a particular scope. 
- Cannot have a mutable reference while we have an immutable one in used. Users of an immutable reference don’t expect the values to change. 
- Multiple immutable references are ok!

#### Not OK
```Rust
  |
4 |     let r1 = &s; // no problem
  |              -- immutable borrow occurs here
5 |     let r2 = &s; // no problem
6 |     let r3 = &mut s; // BIG PROBLEM
  |              ^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("{}, {}, and {}", r1, r2, r3);
  |                                -- immutable borrow later used here
```

#### Ok
```Rust
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```
>The scopes of the immutable references `r1` and `r2` end after the `println!` where they are last used, which is before the mutable reference `r3` is created. These scopes don’t overlap, so this code is allowed.

## Dangling References
> In Rust, the compiler guarantees that references will never be dangling references: if you have a reference to some data, the compiler will ensure that the data will not go out of scope before the reference to the data does.

```Rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

// Return ownership
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

## Slices

### String Slices
A string slice is a reference to part of a String, and it looks like this:

```Rust
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let hello2 = &s[..5];
    let world2 = &s[6..];
```

> String slice range indices must occur at valid UTF-8 character boundaries. If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

### Array Slices
```Rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```