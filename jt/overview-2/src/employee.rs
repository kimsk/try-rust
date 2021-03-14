use crate::ided::Ided;

pub struct Employee {
    pub name: String,
    pub id: i64
}

impl Employee {
    pub fn new(name: String, id: i64) -> Employee {
        Employee { name, id }
    }
    // employee.name()
    // or
    // Employee::name(&employee) // Canonical Form
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> i64 {
        self.id
    }
}

impl Ided for Employee {
    fn my_id(&self) -> i64 {
        self.id
    }
}