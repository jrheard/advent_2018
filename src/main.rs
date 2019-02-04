use std::fs;

fn one_a() -> i32 {
    let contents = fs::read_to_string("src/inputs/1.txt").unwrap();

    contents.lines().map(|x| x.parse::<i32>().unwrap()).sum()
}

fn main() {
    println!("1a: {}", one_a());
}
