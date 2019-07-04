fn main() {
    let a = Some(5);
    let b = None;
    println!("a: {:?}", plus_one(a));
    println!("b: {:?}", plus_one(b));

    let some_u8_value = Some(5u8); // 3u8 is u8 3

    // 1
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // Do nothing
    }

    // 2
    if let Some(5) = some_u8_value {
        println!("five");
    } else {
        println!("not five");
    }

    // 3
    if Option::Some(5) == some_u8_value {
        println!("Five!");
    }

    // 4
    if let Some(value) = some_u8_value {
        println!("Value: {}", value);
    }

    // 5
    if !some_u8_value.is_none() { // or .is_some()
        println!("Is not none!");
    }
}

fn plus_one(opt: Option<i32>) -> Option<i32> {
    match opt {
        Some(val) => Some(val + 1),
        _ => None, // wildcard
        // or None => None
    }
}
