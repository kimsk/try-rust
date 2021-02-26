#![allow(dead_code, unused_imports)]
use std::collections::HashMap;

mod counter_iter;
mod counter_into_iter;

fn main() {
    // for_default();
    // hashmap_default();

    // for_iter();
    // for_iter_and_next();
    // for_into_iter();
    // for_into_iter_and_next();

    // while_let_iter();
    // while_let_into_iter();

    // loop_match_into_iter();
    // loop_match_dbg_into_iter();
    //for_mut()
    counter_iter::counter_iter();
    counter_into_iter::counter_into_iter();
    counter_into_iter::counter_into_iter_reference();
}

// fn for_mut() {
//     let names: Vec<String> = vec!["Pascal".to_string(), "Elvira".to_string()];
//     for name in names {
//         name.push('x');
//         dbg!(name);
//     }
// }

fn loop_match_into_iter() {
    let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];
    let mut names_iter = names.into_iter();
    loop {
            match names_iter.next() {
                Some(name) => println!("{}", name),
                None => break,
        };
    }
}


fn loop_match_dbg_into_iter() {
    let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];
    let mut names_iter = names.into_iter();
    loop {
        let x = match names_iter.next() {
            Some(name) => dbg!(name),//println!("{}", name),
            None => "",//break,
        };
        if x == "" { break };
    }
}

fn while_let_into_iter() {
    let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];
    let mut names_into_iter = names.into_iter();
    //dbg!(names[0]);
    while let Some(name) = names_into_iter.next() {
        dbg!(name);
    }
    //dbg!(names[0]);
}

fn for_into_iter_and_next() {
    let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];
    let mut names_into_iter: std::vec::IntoIter<&str> = names.into_iter();

    let n = names_into_iter.next().unwrap();
    let n = format!("from next {}", n);
    dbg!(n);

    // name is &str
    for name in names_into_iter {
        dbg!(name);
    }
}


// fn for_into_iter_reference() {
//     let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];
//     let names_into_iter: &std::vec::IntoIter<&str> = names.into_iter();
//     for name in names_into_iter {
//         dbg!(name);
//     }
//     dbg!(names[0]);
// }

fn for_into_iter() {
    let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];
    let names_into_iter: std::vec::IntoIter<&str> = names.into_iter();
    // names was moved when calling into_iter
    // dbg!(names[0]);

    // name is &str
    for name in names_into_iter {
        dbg!(name);
    }
}

fn while_let_iter() {
    let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];
    let mut names_iter: std::slice::Iter<&str> = names.iter();
    // name is &&str
    while let Some(name) = names_iter.next() {
        dbg!(name);
    }
    dbg!(names[0]);
}


fn for_iter_and_next() {
    let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];
    let mut names_iter: std::slice::Iter<&str> = names.iter();
    let n = names_iter.next().unwrap();
    let n = format!("from next {}", n);
    dbg!(n);
    // name is &&str
    for name in names_iter {
        dbg!(name);
    }
    dbg!(names[0]);
}

fn for_iter() {
    let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];
    let names_iter: std::slice::Iter<&str> = names.iter();
    // name is &&str
    for name in names_iter {
        dbg!(name);
    }
    dbg!(names[0]);
}


fn hashmap_default() {
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        "Search Inside Yourself".to_string(),
        "A great book about meditation.".to_string(),
    );
    book_reviews.insert(
        "Limitless".to_string(),
        "Unleash the potential of your brain!".to_string(),
    );

    // review is (String, String)
    for review in book_reviews {
        // println!("{}: {}", review.0, review.1);
        dbg!(review);
    }
    
    // book_reviews was moved in the for-loop above!
    // dbg!(book_reviews.get("Limitless"));
}

fn for_default_reference() {
    let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];

    // by default names.into_iter() is called
    // name is &&str
    // impl<'a, T> IntoIterator for &'a Vec<T>
    for name in &names {
        dbg!(name);
    }
    // this is ok
    // impl<'a, T> IntoIterator for &'a Vec<T>
    dbg!(names[0]);
}

fn for_default() {
    let names: Vec<&str> = vec!["Pascal", "Elvira", "Dominic", "Christoph"];

    // by default names.into_iter() is called
    // name is &str
    // impl<T> IntoIterator for Vec<T>
    for name in names {
        dbg!(name);
    }
    // names was moved in the for-loop above!
    // dbg!(names[0]);
}
