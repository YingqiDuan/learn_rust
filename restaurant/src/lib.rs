mod front_of_house;

use crate::front_of_house::hosting;
// use crate::front_of_house::hosting::add_to_waitlist;
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    //add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // meal.seasonal_fruit = String::from('blueberries');

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // crate::hosting::add_to_waitlist();
        // super::hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // struct default private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup, // enum default public
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

use std::fmt;
use std::io;
// use std::fmt::Result;
// use std::io::Result;


fn function1() -> fmt::Result {
    core::fmt::Result::Ok(())
}

fn function2() -> io::Result<()> {
    std::io::Result::Ok(())
}

use std::io::Result as IoResult;

fn function3() -> io::Result<()> {
    std::io::Result::Ok(())
}

// nested paths
// use std::{cmp::Ordering, io};
// use std::io::{self, Write};

use std::collections::*;
// used when testing to bring everything under test into the tests module