# TL;DR

## What is Iterator?
- [Iterator pattern](https://en.wikipedia.org/wiki/Iterator_pattern)
> The **iterator pattern is a design pattern** in which an `iterator` is used to traverse a `container` and access the `container's elements`. **The iterator pattern decouples algorithms from containers**;

> What problems can the Iterator design pattern solve?
The elements of an aggregate object should be accessed and traversed without exposing its representation (data structures).
New traversal operations should be defined for an aggregate object without changing its interface.

> e.g., a `tree` collection can be traversed `DFS` or `BFS`

- [Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html#processing-a-series-of-items-with-iterators)
> The `iterator pattern` allows you to perform some task on a `sequence of items` in turn. **An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished**. 

## Why it's implemented seperately?
- multiple iterators can be implemented for one collection.
- state of iterator has to be mutated to keep track of the position, but traversing should not require the collection to be mutable.

- [Implementing the Iterator trait](https://aloso.github.io/2021/03/09/creating-an-iterator)
> The `iterator trait` is usually not implemented for a `collection` directly. Instead, a new type is created that wraps the collection. 

- [Creating an Iterator in Rust](https://www.reddit.com/r/rust/comments/m0wzw7/creating_an_iterator_in_rust/)
> The iterator state would have to be maintained on the collection, and thus you wouldn't be able to have two users iterate on the collection simultaneously (they need independent state).

> Cloning a dedicated Iterator is actually really cheap (cheaper thn cloning the whole collection) because it just copies a reference and a index rather then the entire collection.

## Iterator in Rust
> We want to use `for loop` with a type that we want. We don't need to know internal of the container, we just want to iterate thru the items inside the container.

```rs
let nums = vec![1, 2, 3];

// bad practice
for i in 0..nums.len() {
    println!("{}", nums[i]);
}

// good practice
for num in &nums {
    println!("{}", num);
}
```
- `bad practice` needs `i` which makes code more complicated, and means we have to know how `nums` work.
- `bad practice` needs bound checking which is less efficient.


1. Rust's `for loop` syntax is actually sugar for iterators (i.e., `IntoIterator::into_iter(values)` & `next()` are called when doing `for v in values`).

1. `into_iter()`, converts the thing implementing `IntoIterator` into an **iterator**.

1. All **Iterators** also implement `IntoIterator`.

# Docs
## [std::iter](https://doc.rust-lang.org/std/iter/)
```rs
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```
> Iterator's full definition includes a number of other methods, e.g., `count`, `nth`, `filter`, `map`.

### [Implementing Iterator](https://doc.rust-lang.org/std/iter/#implementing-iterator)
1. creating a `struct` to hold the iterator's state
```rs
struct MyIterator {
    // keep iterator state
}
```
2. implementing `Iterator`
```rs
// impl Iterator Trait for MyIterator
impl Iterator for MyIterator {
    type Item = ItemType;
    fn next(&mut self) -> Option<Self::Item> {
        // return Some with an item
        // or None to indicate the that no item left
    }
}
```

### [The three forms of iteration](https://doc.rust-lang.org/std/iter/#the-three-forms-of-iteration)
There are three common methods which can create **iterators** from a **collection**:
- `iter()`, which iterates over `&T`.
- `iter_mut()`, which iterates over `&mut T`.
- `into_iter()`, which iterates over `T`.


## [std::iter::IntoIterator](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
There's a trait in the standard library for converting **something** into an iterator: `IntoIterator`. This trait has one method, `into_iter`, which converts the thing implementing `IntoIterator` into an iterator.
```rs
pub trait IntoIterator {
    type Item;
    type IntoIter: Iterator;
    pub fn into_iter(self) -> Self::IntoIter;
}
```
> The standard library also has 
```rs
impl<I: Iterator> IntoIterator for I
```
> which means all **Iterators** also implement `IntoIterator`.

- One benefit of implementing `IntoIterator` is that your type will work with Rust's `for loop` syntax.
- Rust's `for loop` syntax is actually sugar for iterators (i.e., `IntoIterator::into_iter(values)` & `next()` are called when doing `for v in values`). 

## [Iterating by reference](https://doc.rust-lang.org/std/iter/#iterating-by-reference)
`into_iter(self)` consumes (moved) the iterator. To iterate without consuming it, we have to `impl IntoIterator for &T`.

```rs

```

> Conventionally, `iter()` and (mutable) version, `iter_mut()`.

> If a collection type `C` provides `iter()`, it usually also implements `IntoIterator` for `&C`, with an implementation that just calls `iter()`. Likewise, a collection `C` that provides `iter_mut()` generally implements `IntoIterator` for `&mut C` by delegating to `iter_mut()`.


## Players in Iterators
1. `iterators` give you a sequence of values.
1. `iterator adaptors` operate on an `iterator`, producing a new `iterator` with a different output sequence.
    - `map`, `take`, `filter`
1. `consumers` operate on an `iterator`, producing some final set of values.
    - `collect`, `find`, `fold`
    - Consumers are important due to one additional property of iterators, **laziness**



# Links
- [Iterators - Effective Rust](https://doc.rust-lang.org/1.5.0/book/iterators.html)
- [Creating an Iterator in Rust - Mar 2021 - Ludwig Stecher](https://aloso.github.io/2021/03/09/creating-an-iterator)