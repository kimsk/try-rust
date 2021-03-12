# TL;DR

- Every reference has a lifetime and need lifetime parameters (unless lifetime elision rules applys).


## [Lifetime Elision Rules](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision)
> A pattern in code that doesn't need explicit lifetime annotations.

1. Each parameter that is a reference gets its own (*input*) lifetime parameter.
2. If there is exactly one **input lifetime parameter**, that lifetime is assigned to all **output lifetime parameters**.
3. If there are multiple **input lifetime parameters**, but one of them is `&self` or `&mut self` because this is a **method**, the lifetime of `self` is assigned to all output lifetime parameters.

> *The compiler uses three rules to figure out what lifetimes references have when there aren’t explicit annotations.* The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.

