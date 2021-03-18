use overview_2::employee::Employee;

#[derive(Debug)]
struct EmployeeRecords {
    idx: usize,
    employees: Vec<Employee>
}

impl EmployeeRecords {
    pub fn new() -> EmployeeRecords {
        EmployeeRecords {
            idx: 0,
            employees: Vec::new()
        }
    }

    pub fn push(&mut self, employee: Employee) {
        self.employees.push(employee);
    }
}

impl Iterator for EmployeeRecords {
    type Item = Employee;

    fn next(&mut self) -> Option<Self::Item> {
        if self.employees.len() > self.idx {
            let output = self.employees[self.idx].clone();
            self.idx += 1;
            Some(output)
        } else {
            None
        }
    }
}

fn main() {
    let mut employee_records = EmployeeRecords::new();
    employee_records.push(Employee::new(String::from("kk"), 112));
    // Universal Function Call Syntax (UFCS)
    EmployeeRecords::push(&mut employee_records, Employee::new(String::from("crusing"), 113));
    //dbg!(employee_records);
    for employee in employee_records {
        dbg!(employee);
    }
}
