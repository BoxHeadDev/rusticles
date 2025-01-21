// TO DO: Fix the following
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

// TO DO: Define modules and function signatures
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

mod front_of_house {
    mod hosting {
        // pub fn add_to_waitlist() {}
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// TO DO: Why does the following not work?
use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // This is in customer scope not the top level scope
        hosting::add_to_waitlist();
    }
}

// TO DO: Call deliver_order inside fix_incorrect_order
// Use a relative path using a reference to the parent module
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

use crate::front_of_house::hosting; // bring path into scope

// TO DO: Make hosting available to external code
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // TO DO: call add_to_waitlist with an absolute and relative path
    crate::front_of_house::hosting::add_to_waitlist(); // Absolute path
    front_of_house::hosting::add_to_waitlist(); // Relative path

    // TO DO: enable the following by bringing hosting into scope
    hosting::add_to_waitlist();

    // TO DO: Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // TO DO: Change the type of bread to "Wheat"
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    // TO DO: Why does the following not compile?
    // seasonal_fruit is private
    meal.seasonal_fruit = String::from("blueberries");

    // TO DO: Why does the following compile?
    // All variants of Enums are public
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
