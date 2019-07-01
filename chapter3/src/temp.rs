use std::io;
use std::io::prelude::*;

const MENU: &str = "
Temperature Converter
---------------------
1) Celcius to Fahrenheit
2) Fahrenheit to Celcius
3) Exit
";

fn main() {
    loop {
        println!("{}", MENU);
        print!("Choice: ");
        io::stdout().flush().expect("Could not flush stdout!");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line!");
        let choice = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match choice {
            1 => c2f(),
            2 => f2c(),
            3 => break,
            _ => println!("Invalid choice!")
        }
    }
}

fn f2c() {
    loop {
        print!("Temperature: ");
        io::stdout().flush().expect("Could not flush stdout!");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line!");
        let temp = match temp.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("{:?}°F = {:?}°C", temp, f2c_(temp));
        break;
    }
}

fn f2c_(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn c2f() {
    loop {
        print!("Temperature: ");
        io::stdout().flush().expect("Could not flush stdout!");
        let mut temp = String::new();
        io::stdin().read_line(&mut temp).expect("Failed to read line!");
        let temp = match temp.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("{:?}°C = {:?}°F", temp, c2f_(temp));
        break;
    }
}

fn c2f_(temp: f64) -> f64 {
    temp / (5.0 / 9.0) + 32.0
}
