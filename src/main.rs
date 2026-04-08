use std::collections::HashMap; //import required for usage of HashMap
use std::io::{self, Write}; //import required to handle user input and prompt them clearly

//struct built to house the profile of our employees
struct Employee {
    name: String,
    department: String,
}

fn main() {
    //welcome message on startup of program
    println!("--- Welcome to the Company Directory! ---");

    //HashMap allows us to sort employees into a list by department
    let mut directory: HashMap<String, Vec<Employee>> = HashMap::new();

    //loop is used here to keep the program running until the user wants to quit
    loop {
        println!("Please enter a command. (ex: help)");

        //the command var holds the user's command
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line.");

        //removing the enter key newline and making sure our commands aren't case sensitive
        let command = command.trim().to_lowercase();

        if command == "quit" {
            break; //exits the loop and ends the program. data is not stored yet
        } else if command == "help" {
            println!("Commands: help, add, list, quit"); //diplays the list of commands
        } else if command == "list" {
            if directory.is_empty() {
                println!("The directory is empty."); //directory is empty upon startup currently. if the user hasn't added anyone yet the directory will be empty by default
            } else {
                for (dept, employees) in &directory {
                    println!("Department: {}", dept);
                    for emp in employees {
                        println!(" - {}", emp.name); //when there is employees in the directory we will list them
                    }
                }
            }
        } else if command == "add" {
            //collecting the name
            print!("Enter name: ");
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            let name = name.trim().to_string();

            //collecting the department
            print!("Enter department: ");
            io::stdout().flush().unwrap();
            let mut dept = String::new();
            io::stdin().read_line(&mut dept).unwrap();
            let dept = dept.trim().to_string();

            //building our struct. using .clone() so that we don't kill our variables before the println!
            let new_emp = Employee {
                name: name.clone(),
                department: dept.clone(),
            };

            //validating that the addition to the directory was successful
            println!(
                "Added {} to the {} department in the directory!",
                new_emp.name, new_emp.department
            );

            //the entry into the directory. if there is a dept then push, if there isnt then create.
            directory.entry(dept).or_insert(Vec::new()).push(new_emp);
        } else {
            //forces the user to enter a valid command
            println!("Invalid command!")
        }
    }
}
