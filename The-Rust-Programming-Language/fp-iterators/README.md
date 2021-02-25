- [OMG ITERATORS](https://cglab.ca/~abeinges/talks/iter/#0)
- [rust-iterators](https://github.com/rustomax/rust-iterators)
- [Effectively Using Iterators In Rust](https://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html)
- [What is the difference between iter and into_iter?](https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter)
- [The Rust Map Function - A Gateway to Iterators - June 2020 - Konstantin Kostov](https://www.newline.co/@kkostov/the-rust-map-function-a-gateway-to-iterators--f991b22b)
- [Iterators in Rust - October 2020 - Pascal Precht](https://blog.thoughtram.io/iterators-in-rust/)

    - `for` loops consume any type that implement the `Iterator` trait.
    - The `Iterator` trait comes with a `next()` method returning `Option<Self::Item>`.

```rs
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    ...
}
```

    - Iterators are stateful because they keep track of where they are in the iteration process.
    - When there’s a “natural way” to iterate over some type, it can implement the `IntoIterator` trait.

```rs
trait IntoIterator where Self::IntoIter::Item == Self::Item {
    type Item;
    type IntoIter: Iterator;
    fn into_iter(self) -> Self::IntoIter;
    ...
}
```
    - Any type that implements `IntoIterator` is also called an **Iterable**.