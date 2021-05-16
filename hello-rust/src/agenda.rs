/*

Using a hash map and vectors, create a text interface to allow a user to add employee names to
a department in a company.

For example, “Add Sally to Engineering” or “Add Amir to Sales.”

Then let the user retrieve a list of all people in a department or all people in the company by
department, sorted alphabetically.
 */
use std::io;
use std::collections::HashMap;

enum Commands {
    Add(String, String),
    ListByDepartment(String),
    ListEntireCompany,
}


fn parse_line(line: &str) -> Option<Commands> {
    if line.is_empty() {
        return None
    }

    let split = line.split_ascii_whitespace().collect::<Vec<&str>>();

    return match split[..] {
        ["Add", person, "To", department] => {
            let add = Commands::Add(String::from(person), String::from(department));
            Some(add)
        },
        ["List", department] => Some(Commands::ListByDepartment(String::from(department))),
        ["List"] => Some(Commands::ListEntireCompany),
        _ => None,
    }
}


pub fn agenda() {
    println!("################");
    println!("Welcome to Agenda!");
    println!("################");
    println!("Please type your command below:");
    let mut company = HashMap::new();

    loop {
        // Parse input
        let mut line= String::new();
        io::stdin().read_line(&mut line);
        let line = line.trim();

        match parse_line(line) {
            Some(Commands::Add(person, department)) => {
                let mut employees = company.entry(department).or_insert_with(Vec::new);
                employees.push(person);
            }
            Some(Commands::ListByDepartment(department)) => {
                match company.get(&department) {
                    None => { println!("Department does not exist: {}", department) }
                    Some(employees) => {
                        print_employees_of_department(&department, employees)
                    }
                }
            }
            Some(Commands::ListEntireCompany) => {
                for (department, employees) in &company {
                    print_employees_of_department(department, employees);
                }
            }
            None => {
                println!("Unknown command '{}', please try again.", line)
            }
        }
    }
}

fn print_employees_of_department(department: &String, employees: &Vec<String>) {
    println!("Employees of department: {}", department);
    let mut sorted_employees = employees.clone();
    sorted_employees.sort();
    for employee in sorted_employees {
        println!("- {}", employee);
    }
}
