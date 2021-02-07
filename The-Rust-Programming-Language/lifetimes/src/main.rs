#![allow(dead_code, unused_variables, unused_assignments)]
// every reference has lifetime (usually inferred)
// Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

// The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.

// The Rust compiler has a borrow checker that compares scopes to determine whether all borrows are valid.

// Lifetime annotations don’t change how long any of the references live. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter. Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

// One special lifetime we need to discuss is 'static, which means that this reference can live for the entire duration of the program. All string literals have the 'static lifetime

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    {
        // string1 is valid until the end of the outer scope, string2 is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope. 
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // let result;
    // {
    //     // The error shows that for result to be valid for the println! statement, string2 would need to be valid until the end of the outer scope.
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str()); // `string2` does not live long enough
    // }
    // println!("The longest string is {}", result);
}

// even though we’ve specified a lifetime parameter 'a for the return type, this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all.
// fn longest_dangling<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

fn declare_lifetime<'a>() {
    let x: &i32;        // a reference
    let x: &'a i32;     // a reference with an explicit lifetime
    let x: &'a mut i32; // a mutable reference with an explicit lifetime
}

// The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in. 

// Rust can analyze the code within the function without any help. However, when a function has references to or from code outside that function, it becomes almost impossible for Rust to figure out the lifetimes of the parameters or return values on its own. The lifetimes might be different each time the function is called. This is why we need to annotate the lifetimes manually.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


fn valid() {
    {
        let x = 5;            // ----------+-- 'b
                            
        //           |
        let r = &x;           // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+
}

// At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b. The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.
// fn invalid() {
//     {
//         let r;                // ---------+-- 'a
//                               //          |
//         {                     //          |
//             let x = 5;        // -+-- 'b  |
//             r = &x;           //  |       |
//         }                     // -+       |
//                               //          |
//         println!("r: {}", r); //          |
//     }                         // ---------+
// }

// The patterns programmed into Rust’s analysis of references are called the lifetime elision rules. These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

// The elision rules:
// 1. each parameter that is a reference gets its own lifetime parameter. 
// 2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
// 3. (for method signature only) if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
fn first_word(s: &str) -> &str {
    //fn first_word<'a>(s: &'a str) -> &'a str {
    
        let bytes = s.as_bytes();
    
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
    
        &s[..]
    }


fn struct_lifetime() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let i: ImportantExcerpt;
    {
        // let novel = String::from("Call me Ishmael. Some years ago..."); // novel can't be here as it does not live long enough when println!
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    println!("{:?}", i.part);
}

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // output lifetime is a lifetime of self
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn struct_lifetime2() {
    let ann;
    {
        let i = ImportantExcerpt {
            part: "test"
        };

        ann = i.announce_and_return_part("test2");
    }
    // ann is accessed here, but i lifetime already ends.
//    println!("{}", ann);
}


use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}