use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;

// Rust groups errors into two major categories: recoverable and unrecoverable errors.

// Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. Rust doesn’t have exceptions. Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.

// By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. 

// The alternative is to immediately abort, which ends the program without cleaning up. Memory that the program was using will then need to be cleaned up by the operating system. 

// returning Result is a good default choice when you’re defining a function that might fail.

// Use panic!, unwrap, expect when writing Examples, Prototype Code, and Tests as it's cleaner (only code you wanna show, no error-handling code)

// panic! is how a test is marked as a failure, calling unwrap or expect is exactly what should happen.

// If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file.

// RUST_BACKTRACE=1 cargo run
fn main() {
    // panic();
    // panic2();
    open_file();
}

fn read_username_from_file_fs() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

fn read_username_from_file3() -> Result<String, Error> {
    let mut s = String::new();

    // chain ?
    File::open("hello.txt")?
        .read_to_string(&mut s)?;

    Ok(s)
}

fn read_username_from_file_unwrap() -> Result<String, Error>  {
    let mut f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|_| {
                //return Err(error); this doesn't work, can't exit early
                panic!();
            })
        } else {
            // return Err(error); this doesn't work, can't exit early
            panic!();
        }
    });
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap_or_else(|_| panic!());
    Ok(s)
}

// This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier.

// Unlike unwrap_or_else, error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert errors from one type into another. 

// the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `std::ops::Try`)
fn read_username_from_file2() -> Result<String, Error> {
    // If the value of the Result from File::open is an Ok, the value inside the Ok will be assigned to f from this expression, and the program will continue. 
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// We chose io::Error as the return type of this function because that happens to be the type of the error value returned from both of the operations we’re calling in this function’s body that might fail: the File::open function and the read_to_string method.
fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // f has to be mutable
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn unwrap_expect() {
    let f = File::open("hello.txt")
        .unwrap();

    let f = File::open("hello.txt")
        .expect("Failed to open hello.txt");

}

fn open_file_unwrap_or_else() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn open_file() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

fn panic2() {
    let v = vec![1, 2, 3];

    v[99];
}

fn panic() {
    panic!("crash and burn!");
}
