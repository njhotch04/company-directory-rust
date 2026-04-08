use std::collections::HashMap;
use std::io;

struct Employee {
    name: String,
    department: String,
    email: String,
    phone: String,
    remote: bool,
}

fn main() {
    println!("--- Welcome to the Company Directory! ---");

    let mut directory: HashMap<String, Vec<Employee>> = HashMap::new();

    loop {
        println!("Please enter a command.");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line.");

        let command = command.trim();

        if command == "quit" {
            break;
        }
    }
}
