fn main() {
    // Create using associated function
    let mut v1: Vec<i32> = Vec::new();

    // Create using macro
    let v2 = vec![1, 2, 3];

    // Mutation
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.pop();
    println!("v1: {:?}\nv2: {:?}", v1, v2);

    // Access type 1
    let second: &i32 = &v1[1];
    println!("Second item of v1: {}", second);

    // Access type 2
    // Return: Option<&T>
    match v2.get(2) {
        Some(thing) => println!("Third item of v2: {}", thing),
        None => println!("No third item available.")
    };

    // Iteration, mutable
    for i in &mut v1 {
        *i += 50; // Dereference the `&mut i32` and add 50 to that element
    }

    // Iteration, immutable
    for i in &v1 {
        println!("{}", i);
    }

    // Holding values of different types using an enum
    #[derive(Debug)]
    enum Cell {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![
        Cell::Int(69),
        Cell::Float(3.14),
        Cell::Text("Reps for Jesus".to_string())
    ];
    for i in row.iter() {
        println!("{:?}", i);
    }
}
