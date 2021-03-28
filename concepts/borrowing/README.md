# Rules
1. any **borrow** must last for a scope no greater than that of the **owner**.
1. you may have one or the other of these two kinds of borrows, but not both at the same time:
    - one or more references `&T` to a resource,
    - exactly one mutable reference `&mut T`.

# Data Race
There is a ‘data race’ when two or more pointers access the same memory location at the same time, where at least one of them is writing, and the operations are not synchronized.

- Iterator invalidation
> happens when you try to mutate a collection that you’re iterating over. 

- Use after free
> references live longer than the resource they refer to.

# Links
- [References and Borrowing](https://doc.rust-lang.org/1.5.0/book/references-and-borrowing.html)