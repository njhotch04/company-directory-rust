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

    println!("Employee Name: {}", employee1.name);
    println!("Department: {}", employee1.department);
    println!("Email Address: {}", employee1.email);
    println!("Phone #: {}", employee1.phone);
    println!("Remote Employee? {}", employee1.remote);
}
