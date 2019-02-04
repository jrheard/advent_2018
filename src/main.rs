use std::collections::HashSet;
use std::fs;

fn one_a() -> i32 {
    let contents = fs::read_to_string("src/inputs/1.txt").unwrap();
    contents.lines().map(|x| x.parse::<i32>().unwrap()).sum()
}

// You notice that the device repeats the same frequency change list over and over.
// To calibrate the device, you need to find the first frequency it reaches twice.
fn one_b() -> i32 {
    let contents = fs::read_to_string("src/inputs/1.txt").unwrap();
    let reductions = contents
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        // "Note that your device might need to repeat its list of frequency changes many
        // times before a duplicate frequency is found."
        .cycle()
        .scan(0, |state, x| {
            *state = *state + x;
            Some(*state)
        });

    let mut seen_frequencies: HashSet<i32> = HashSet::new();
    for frequency in reductions {
        if seen_frequencies.contains(&frequency) {
            return frequency;
        } else {
            seen_frequencies.insert(frequency);
        }
    }

    -1
}

fn main() {
    println!("1a: {}", one_a());
    println!("1b: {}", one_b());
}
