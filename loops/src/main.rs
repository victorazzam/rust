fn main() {
    loop {
        println!("Again!");
        break;
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter > 10 {
            break counter * 2;
        }
    };
    println!("result = {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFT OFF!");

    let a = [10, 20, 30, 40, 50];
    for item in a.iter() {
        println!("{}", item);
    }

    for i in (1..4).rev() {
        print!("{} ", i)
    }
    println!("LIFT OFF!");
}
