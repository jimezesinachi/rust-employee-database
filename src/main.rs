use regex::Regex;
use std::{collections::HashMap, io};

fn main() {
    let mut employee_database: Vec<HashMap<String, String>> = Vec::new();

    println!("Welcome to the employee database!");

    fn add_employee(database_vector: &mut Vec<HashMap<String, String>>) {
        let mut input = String::new();

        println!(
            "Please input the name and department of the employee you want to add to the database"
        );
        println!("For example, 'Add Sally to Engineering', 'Add Amir to Sales':");

        io::stdin()
            .read_line(&mut input)
            .expect("Enter a valid input!");

        let regex_parser =
            Regex::new(r"^Add ([A-Za-z]{1}[A-Za-z-' ]{1,99}) to ([A-Z]{1}[A-Za-z-]{1,99})$")
                .unwrap();

        let input_str_slice = input.clone().trim_end().to_string();

        let valid = regex_parser.is_match(&input_str_slice);

        match valid {
            true => println!("Valid input!"),
            false => {
                println!("Invalid input!");
                return;
            }
        }

        for caps in regex_parser.captures_iter(&input_str_slice) {
            println!(
                "name: {}, department: {}",
                caps.get(1).unwrap().as_str(),
                caps.get(2).unwrap().as_str(),
            );

            let mut map: HashMap<String, String> = HashMap::new();

            map.insert(
                String::from("name"),
                String::from(caps.get(1).unwrap().as_str()),
            );

            map.insert(
                String::from("department"),
                String::from(caps.get(2).unwrap().as_str()),
            );

            database_vector.push(map);
        }

        println!(
            "Here's the updated employee database: {:?}",
            database_vector
        );
    }

    fn get_all_employees(database_vector: &Vec<HashMap<String, String>>) {
        println!("Here's the entire employee database: {:?}", database_vector);
    }

    fn get_all_employees_in_a_single_department(
        database_vector: &mut Vec<HashMap<String, String>>,
        input: &str,
    ) {
        let mut vec_by_department = Vec::new();

        for val in database_vector {
            if val.get("department") == Some(&String::from(input)) {
                vec_by_department.push(val.clone());
            }
        }

        println!(
            "Here's the list of employees in the '{input}' department: {:?}",
            vec_by_department
        );
    }

    loop {
        println!("To add a new employee to the database, press '1' and hit enter/return:");
        println!("To get all the employees in a single department, type in the name of the department, then hit enter/return");
        println!("To get all the employees in all departments in the database, press '2' and hit enter/return:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Enter a valid input!");

        let trimmed_input = input.trim_end();

        if trimmed_input == "1" {
            add_employee(&mut employee_database)
        } else if trimmed_input == "2" {
            get_all_employees(&employee_database)
        } else {
            get_all_employees_in_a_single_department(&mut employee_database, &trimmed_input)
        }
    }
}
