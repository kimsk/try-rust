use std::{error::Error, fmt, sync::Arc};

use parking_lot::Mutex;

#[test]
fn my_test() {
    assert!(true);
}

fn greet(x: i64) {
    println!("Hello to number {}", x);
}

fn greet_str(x: i64, y: &str) -> String {
    format!("Hello to &str {} {}", x, y)
}

struct Employee {
    name: String,
    id: i64,
}

impl Employee {
    fn new(name: String, id: i64) -> Employee {
        // struct now owns String
        let employee = Employee{ name, id };
        //println!("{}", name); // employee already took the ownership of `name`, println! can't take it
        println!("{}", employee.name); 
        // above code is ok, https://stackoverflow.com/questions/30450399/does-println-borrow-or-own-the-variable
        // The macros print!, println!, eprint!, eprintln!, write!, writeln! and format! are a special case and implicitly take a reference to any arguments to be formatted.
        println!("{}", &employee.name); 

        println!("{}", employee.name.clone());
        println!("{}", employee.name());
        // let x = employee.name; // name is moved (employee is partially moved), lifetime is not OK
        let x = &employee.name;// borrow is ok
        employee
    }

    // fn name(&self) -> &str {
    fn name<'a>(&'a self) -> &'a str {
        &self.name // return &str, borrowed not moved
        // &self.name = &(self.name)
        // &self.name != (&self).name
    }

    // fn name_moved(&self) -> String {
    //     self.name // return String, can't moved!
    //     // cannot move out of `self.name` which is behind a shared reference
    // }

    fn name_cloned(&self) -> String {
        self.name.clone() // have to clone explicitly
    }

    fn id(&self) -> i64 {
       self.id 
    }
}

impl fmt::Display for Employee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " name: {}, id: {}", self.name, self.id)
    }
}

enum IsTrue {
    True(i64),
    False,
}

fn before_result() {
    greet(100);
    let str = "str";
    let mut string = str.to_string();

    string.push('i');
    string.push('n');
    string.push('g');
    greet_str(100, str);
    let new_string = greet_str(200, &string);
    println!("new string {}", new_string);

    let employee = Employee{
        name: "Stream".to_string(),
        id: 101
    };

    let employee = Employee::new(String::from("Stream"), 101);
    println!("My greeting: {}", employee);

    println!("{}", employee.name());
    let my_value = IsTrue::False;

    match my_value {
        IsTrue::True(x) => println!("true({})!", x),
        IsTrue::False => println!("false!"),
    }
}

enum MyResult<Success, Failure> {
    Ok(Success),
    Err(Failure),
}

// Box: something on the heap
// dyn Error: something implements Error trait 
fn process() -> Result<String, Box<dyn Error>> {
    Ok(String::from("yay!"))
}

fn result() -> Result<(), Box<dyn Error>> {
    let result: MyResult<i64, String>;
    let result = process();

    // match result {
    //     Ok(x) => {
    //         println!("My result {}", x);
    //     },
    //     Err(_err) => {
    //         println!("It errored!");
    //     }
    // }
    // we can use `?` to replace code above
    println!("My result: {}", result?);
    return Ok(());
}

// employee ownership is taken over 
fn move_thing(employee: Employee) {
    println!("{}", employee.name);
}

fn borrow_thing<'a>(employee: &'a Employee) -> () {
    println!("{}", employee.name);
}

fn main() -> Result<(), Box<dyn Error>> {
    before_result();

    result();

    let e = Employee::new(String::from("test"), 42);
    borrow_thing(&e);
    println!("{}", e.name());
    move_thing(e);
    //println!("{}", e.name()); // e was moved

    // Mutex & Arc
    let employee = Employee::new(String::from("test"), 42);
    let locked_employee = Mutex::new(employee);

    //println!("{}", employee.name); // employee was moved

    let unlocked_employee = locked_employee.lock();
    println!("{}", unlocked_employee.name()); // this is ok

    let employee = Employee::new(String::from("test"), 42);
    let reference_counted = Arc::new(employee);
    

    Ok(())
}
