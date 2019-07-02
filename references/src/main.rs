fn main() {
    let mut s = String::from("Hello"); // Create mutable String object
    change(&mut s);                    // Pass object with &mut to change()
    println!("{}", s);                 // Print the final value

    let r1 = &s;                       // Only 1 mutable reference allowed
    let r2 = &s;                       // concurrently in a given scope for
    println!("{}, {}", r1, r2);        // a particular piece of data.

    let r3 = &mut s;                   // Works because the last reference
    println!("{}", r3);                // is no longer used, so it is gone
}

fn change(data: &mut String) {         // Accept mutable String reference
    data.push_str(", world!");         // Modify referenced data
}
