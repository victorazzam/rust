mod front_of_house; // Body in file named the same way

mod back_of_house {
    // All variants become public
    pub enum Appetizer {
        Soup,
        Salad
    }
    // All fields are private by default, even if the struct is public
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: toast.to_string(),
                seasonal_fruit: "peaches".to_string()
            }
        }
    }
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}
}

// Path lists to combine use statements
// use std::{cmp::Ordering, io};
// use std::io::{self, Write};

// use crate::front_of_house::hosting;
// use self::front_of_house::hosting;
// use front_of_house::hosting;
//
// Brings name `hosting` (renamed to `host`) into caller's scope
pub use front_of_house::hosting as host;

fn serve_order() {}

fn eat() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Brought into scope
    host::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = "Wheat".to_string();
    println!("I'd like {} toast please.", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;
}
