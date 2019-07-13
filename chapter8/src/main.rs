use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

const MENU: &str = "
Menu
----
1. Add employee
2. Remove employee
3. List employees
4. Exit
";

fn main() {
    let mut db = HashMap::new();
    db.insert("Sales".to_string(),
        vec!["Adam Sandler".to_string(), "Brad Pitt".to_string()]);
    db.insert("HR".to_string(),
        vec!["Doug Stanhope".to_string(), "Kumail Nanjiani".to_string()]);
    println!("Enter 0 to exit.");
    loop {
        print!("{}\n> ", MENU);
        io::stdout().flush().expect("Failed to flush stdout!");
        let mut input = String::with_capacity(32);
        io::stdin().read_line(&mut input).expect("Failed to read line!");
        match input.trim().parse().unwrap_or(0) {
            1 => add_employee(&mut db),
            2 => del_employee(&mut db),
            3 => view_db(&mut db),
            4 => break,
            _ => println!("Look at the fucking menu, you epileptic ape!")
        }
        //let args: Vec<&str> = input.to_lowercase().trim().split(' ').collect();
        //if args.len() != 4 { continue; }
        //let data = (args[0], args[1], args[2], args[3]);
        //match data {
            //("", _) => println!("{:?}", data),
            //_ => println!("123")
        //};
    }
}

fn add_employee(db: &mut HashMap<String, Vec<String>>) {
    print!("Name of employee: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line!");

    print!("Which department: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    let mut dept = String::new();
    io::stdin().read_line(&mut dept).expect("Failed to read line!");

    let entry = db.entry(dept.trim().to_string()).or_insert(Vec::new());
    entry.push(name.trim().to_string());
}

fn del_employee(db: &mut HashMap<String, Vec<String>>) {
    print!("Name of employee: ");
    io::stdout().flush().expect("Failed to flush stdout!");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line!");
    name = name.trim().to_string();

    for (k, v) in db {
        if v.contains(&name) {
            for i in 0..v.len() {
                if &name == v.get(i).unwrap() {
                    v.remove(i);
                    println!("Removed {} from {}", name, k);
                }
            }
        }
    }

    // Failed attempt
    //if let Some(x) = db.get_mut(&dept) {
        //for item in x.iter() {
            //if &name == item {
                //x.remove(x.iter().position(|x| *x == name).unwrap());
                //println!("Removed {} from {}", name, dept.0);
            //}
        //}
    //}

    // Failed attempt
    //match names.iter().position(|x| x == &name) {
        //Some(val) => {
            //names.remove(val);
            //println!("Removed {} from {}", name, dept);
        //},
        //None => continue
    //};
}

fn view_db(db: &HashMap<String, Vec<String>>) {
    for (dept, names) in db.iter() {
        println!("\n{}\n{}", dept, "-".repeat(dept.len()));
        for name in names.iter() {
            println!("{}", name);
        }
    }
}
