use std::collections::HashMap; //import required for usage of HashMap
use std::io::{self, Write}; //import required to handle user input and prompt them clearly

//struct built to house the profile of our Employees
#[derive(Debug, Clone)]
struct Employee {
    name: String,
    department: String,
}

fn main() {
    println!("--- Welcome to the Company Directory! ---"); //proof of startup of our program only to display once

    let mut directory: HashMap<String, Vec<Employee>> = HashMap::new();

    loop {
        println!("Please enter a command. (ex: help)");

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line.");

        let command = command.trim().to_lowercase();

        if command == "quit" {
            break;
        } else if command == "help" {
            println!("Commands: help, add, list, quit");
        } else if command == "list" {
            println!("{:#?}", directory)
        } else if command == "add" {
            print!("Enter name: ");
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_string();

            print!("Enter department: ");
            io::stdout().flush().unwrap();
            let mut dept = String::new();
            io::stdin().read_line(&mut dept).unwrap();
            let dept = dept.trim().to_string();

            let new_emp = Employee {
                name: name.clone(),
                department: dept.clone(),
            };

            println!(
                "Added {} to the {} department in the directory.",
                new_emp.name, new_emp.department
            );

            directory.entry(dept).or_insert(Vec::new()).push(new_emp);
        } else {
            println!("Invalid command!")
        }
    }
}
