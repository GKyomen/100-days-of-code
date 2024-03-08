use std::{collections::HashMap, io};

fn main() {
    println!("Welcome to the company employees database.");
    let mut database: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("Please select an option:");
        println!("0 - Exit the program");
        println!("1 - Add a new employee");
        println!("2 - List all employees");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        let option: u32 = match option.trim().parse() {
            Ok(num) => {
                if num > 2 {
                    println!("Please enter exactly one of the options. Returning to the menu.");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Please enter exactly one of the options. Returning to the menu.");
                continue;
            }
        };

        match option {
            0 => {
                println!("Thanks for using this program. Bye!");
                break;
            }
            1 => 'add: {
                let mut command = String::new();
                println!("Insert the name and department in the following pattern:");
                println!("Add 'name' to 'department'");

                io::stdin()
                    .read_line(&mut command)
                    .expect("Failed to read line");

                let split: Vec<&str> = command.trim().split_whitespace().collect();
                if split.len() != 4 {
                    println!("Wrong command format. Returning to menu.")
                }
                let mut employee = String::new();
                let mut department = String::new();
                for (i, word) in split.iter().enumerate() {
                    if (i == 0 && *word != "Add") || (i == 2 && *word != "to") {
                        println!("Wrong command format. Returning to menu.");
                        break 'add;
                    }
                    if i == 1 {
                        employee = (*word).to_string();
                    }
                    if i == 3 {
                        department = (*word).to_string();
                    }
                }
                add_employee(&mut database, employee, department);
            }
            2 => list_employees(&mut database),
            _ => {
                println!("Unexpected error. Returning to menu");
                continue;
            }
        }
    }
}

fn add_employee(database: &mut HashMap<String, Vec<String>>, employee: String, department: String) {
    database
        .entry(department)
        .and_modify(|d| d.push(employee.clone()))
        .or_insert(vec![employee]);
}

fn list_employees(database: &mut HashMap<String, Vec<String>>) {
    let mut departments: Vec<_> = database.keys().cloned().collect();
    departments.sort();

    for department in departments {
        if let Some(employees) = database.get_mut(&department) {
            employees.sort();
            println!("Employees in department {}: {:?}", department, employees);
        }
    }
}
