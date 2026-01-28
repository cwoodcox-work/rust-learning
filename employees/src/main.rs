use std::io;
use std::collections::HashMap;

fn main() {
    let mut map = enterEmployees();
    chooseReport(&mut map);
}

fn enterEmployees() -> HashMap<String,Vec<String>> {
    println!("Add Employees to Departments");
    println!("Example use: 'Add Sally to Engineering' or 'Add Amir to Sales'");
    println!("When finished, type 'Department' or 'People' to receive reports. Type 'Both' to receive both reports.");
    
    let mut names: Vec<String> = Vec::new();
    let mut departments: Vec<String> = Vec::new(); 
    let mut map: HashMap<String,Vec<String>> = HashMap::new();

    loop{

        println!("Please input employee and department or type 'Done':");

        let mut userInput = String::new();
        let mut splitInput: Vec<String> = Vec::new();

        io::stdin()
            .read_line(&mut userInput)
            .expect("Failed to read line");

        for word in userInput.split_whitespace() {
            splitInput.push(word.to_lowercase());            
        }
        if splitInput.len()<4 {
            break;
        }
        let name = &splitInput[1];
        let function = &splitInput[0];
        let department = &splitInput[3];
        map.entry(department.clone()).and_modify(|dNames| dNames.push(name.clone())).or_insert(vec![name.clone()]);
        names.push(name.clone());
        departments.push(department.clone());

        
    }
    return map;
}

fn chooseReport(map: &mut HashMap<String,Vec<String>>) {
    println!("Type the name of department for list of people in that department");
    println!("Type 'People' for list of all employees");
    println!("Enter report type: ");
    let mut reportType = String::new();

    io::stdin()
        .read_line(&mut reportType)
        .expect("Failed to read line");

    let reportType = reportType.trim();
    if reportType == "People" {
        company(map);
    }
    else {
        department_faculty(map,reportType);
    }

}

fn company(map: &mut HashMap<String,Vec<String>>) {
    for (key,mut val) in map {
        println!("{key}");
        val.sort();
        for name in val {
            println!("{name}");
        }
    }
}

fn department_faculty(map: &mut HashMap<String,Vec<String>>,rType: &str) {
    let mut nameList: &mut Vec<String> = match map.get_mut(rType) {
        Some(i) => i,
        None => panic!(),
    };
    nameList.sort();
    for name in nameList {
        println!("{name}");
    }
}
