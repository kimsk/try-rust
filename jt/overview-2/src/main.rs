#![allow(dead_code, unused_imports)]

use std::{error::Error, fmt::Display};

mod employee;
use employee::Employee;

mod student;
use student::Student;

mod ided;
use ided::{Ided, use_ided, use_ided_box, use_ided_generic};

/// orphan rule
/// only traits defined in the current crate can be implemented for arbitrary types
/// impl doesn't use only types from inside the current crate
/// ```
/// impl fmt::Display for i64 {}
/// ```

// Ided in the crate, this is ok
impl Ided for i64 {
    fn my_id(&self) -> i64 {
        *self
    }
}

// Box<dyn Error>>
// Box: something on the heap
// dyn: we don't know what it is
// Error: that implements the Error trait
fn main() -> Result<(), Box<dyn Error>> {
    let mut employee = Employee::new(String::from("kk"), 112);
    dbg!(employee.name());
    dbg!(Employee::name(&employee));
    employee.name = String::from("kk crusing");

    // Employee & i64 has nothing to do with each other, but use_ided can process it!
    use_ided(employee);
    use_ided(112);
    dbg!(112.my_id());

    let i64_on_heap = Box::new(112);
    use_ided_box(i64_on_heap);

    let student = Student { student_id: 112 };
    use_ided_generic(student);
    Ok(())
}

