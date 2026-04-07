use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Employee {
    name: String,
    department: String,
    email: String,
    phone: String,
    remote: bool,
}

fn main() {
    let employee1 = Employee {
        name: String::from("Nathan Hotchkiss"),
        department: String::from("Repair"),
        email: String::from("njhotch04@gmail.com"),
        phone: String::from("207-XXX-XXXX"),
        remote: true,
    };

    let employee2 = Employee {
        name: String::from("Ron Hotchkiss"),
        department: String::from("Installation"),
        email: String::from("XXXXXXXX@gmail.com"),
        phone: String::from("207-XXX-XXXX"),
        remote: false,
    };

    let mut directory: HashMap<String, Vec<Employee>> = HashMap::new();

    directory
        .entry(employee1.department.clone())
        .or_insert(Vec::new())
        .push(employee1);

    directory
        .entry(employee2.department.clone())
        .or_insert(Vec::new())
        .push(employee2);

    println!("{:#?}", directory);
}
