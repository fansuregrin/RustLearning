// Using Nested Paths to Clean Up Large use Lists
// use std::{cmp::Ordering, io};
// use std::io::{self, Write};
//
// The Glob Operator
// use std::collections::*;
//
// Providing New Names with the as Keyword
// use std::io::Result as IoResult;


#[allow(unused)]
mod front_of_house;

// Adding use and a path in a scope is similar to creating a symbolic link in the filesystem
use crate::front_of_house::hosting;

#[allow(unused)]
pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    //
    // relative path
    // front_of_house::hosting::add_to_waitlist();
    // 
    // with `use`
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

#[allow(unused)]
mod back_of_house;
