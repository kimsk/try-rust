# Keys
Ownership is how Rust achieves its largest goal, memory safety. This is done at compile-time with zero-cost abstraction.

# Move Semantic
> new binding takes ownership either via new variable or function parameter.
```rs
// v, vector object, is on stack, content of the vector is on the heap
let v: Vec<i32> = vec![1, 2, 3];

// v2, vector object, is on stack and points to the same content of the vector of the heap
// v is not valid anymore as it's not safe (i.e., data race) to have two pointers point to the same content!
let v2 = v;
```

# Copy Traits
> All primitive types implement the Copy trait and their ownership is therefore not moved


# Links
- [Ownership](https://doc.rust-lang.org/1.5.0/book/ownership.html)