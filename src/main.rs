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
        department: String::from("Repair"),
        email: String::from("XXXXXXXX@gmail.com"),
        phone: String::from("207-XXX-XXXX"),
        remote: false,
    };

    let mut directory = HashMap::new();

    directory.insert(employee1.name.clone(), employee1);
    directory.insert(employee2.name.clone(), employee2);

    println!("{:#?}", directory);
}
