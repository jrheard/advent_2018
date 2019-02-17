#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]

// Rustfmt doesn't let me alphabetize these, oh well!
mod eight;
mod eleven;
mod five;
mod four;
mod nine;
mod one;
mod seven;
mod six;
mod ten;
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
    println!("6b: {}", six::six_b());
    println!("7a: {}", seven::seven_a());
    println!("7b: {}", seven::seven_b());
    println!("8a: {}", eight::eight_a());
    println!("8b: {}", eight::eight_b());
    println!("9a: {}", nine::nine_a());
    println!("9b: {}", nine::nine_b());
    // ten::ten() prints out 10a as a side effect and returns 10b.
    println!("10a:");
    let ten_b = ten::ten();
    println!("10b: {}", ten_b);
    println!("11a: {:?}", eleven::eleven_a());
}
