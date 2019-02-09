#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::unreadable_literal, clippy::needless_range_loop)
)]

// these imports appear to be global to all other files in the project?
extern crate serde;

#[macro_use]
extern crate serde_scan;

#[macro_use]
extern crate serde_derive;

// rustfmt doesn't let me alphabetize these, oh well!
mod five;
mod four;
mod one;
mod six;
mod three;
mod two;
mod util;

fn main() {
    println!("1a: {}", one::one_a());
    println!("1b: {}", one::one_b());
    println!("2a: {}", two::two_a());
    println!("2b: {}", two::two_b());
    println!("3a: {}", three::three_a());
    println!("3b: {}", three::three_b());
    println!("4a: {}", four::four_a());
    println!("4b: {}", four::four_b());
    println!("5a: {}", five::five_a());
    println!("5b: {}", five::five_b());
    println!("6a: {}", six::six_a());
}
