// https://stackoverflow.com/questions/48037128/how-to-use-the-same-iterator-twice-once-for-counting-and-once-for-iteration
pub fn twice_reference() {
    dbg!("play_with_iterators::twice_reference()");
    let strings: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];

    let mut count = 0;
    // impl IntoIterator for &T
    for s in &strings {
        println!("{}", s);
        count = count + 1;
    }
    let strings = dbg!(strings);

    // impl IntoIterator for T
    for s in strings {
        println!("{}", s);
    }
    // dbg!(strings);
    dbg!(count);
}

// https://doc.rust-lang.org/std/iter/#iterating-by-reference
// While many collections offer iter(), not all offer iter_mut(). 
// For example, mutating the keys of a HashSet<T> or HashMap<K, V> 
// could put the collection into an inconsistent state if the key hashes change,
// so these collections only offer iter().
pub fn for_iterators_vec() {
    dbg!("play_with_iterators::for_iterators_vec()");
    let strings: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    // impl<'a, T> IntoIterator for &'a Vec<T>
    println!("impl<'a, T> IntoIterator for &'a Vec<T>, same as iter()");
    for s in &strings {
        dbg!(s);
    }
    let strings = dbg!(strings);

    // impl<T> IntoIterator for Vec<T>
    println!("impl<T> IntoIterator for Vec<T>");
    for s in strings {
        dbg!(s);
    }

    let mut strings: Vec<String> = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    // impl<'a, T> IntoIterator for &'a mut Vec<T>
    println!("impl<'a, T> IntoIterator for &'a mut Vec<T>, same as iter_mut()");
    for s in &mut strings {
        s.push_str("_mutated");
        dbg!(s);
    }
    dbg!(strings);
}

// https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter/34745885#34745885
// It's not possible for the array to implement an iterator that yields values
// because it cannot "shrink" to give up its items.
pub fn for_iterators_array() {
    dbg!("play_with_iterators::for_iterators_array()");
    let array: [i32;3] = [1, 2, 3];

    // impl<'a, T> IntoIterator for &'a [T;n]
    println!("impl<'a, T> IntoIterator for &'a [T;n]");
    for a in &array {
        dbg!(a);
    }

    // impl<T> IntoIterator for [T;n]
    // the trait `Iterator` is not implemented for `[i32; 3]`
    // arrays are not iterators, but slices like the following are: `&[1, 2, 3]`
    // println!("impl<T> IntoIterator for [T;n]");
    // for a in array {
    //     dbg!(a);
    // }

    let mut array = dbg!(array);

    // impl<'a, T> IntoIterator for &'a [T;n]
    println!("impl<'a, T> IntoIterator for &'a mut Vec<T>, same as iter_mut()");
    for a in &mut array {
        *a += 1;
        dbg!(a);
    }
    dbg!(array);
}