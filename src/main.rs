use hashbrown::HashMap;
use hashbrown::HashSet;
use itertools::Itertools;

// TODO: i don't understand anything about serde, serde_derive, serde_scan, or macro importing
// i just found out about serde_derive through a reddit comment, it has a useful scan! macro
extern crate serde;

#[macro_use]
extern crate serde_scan;

#[macro_use]
extern crate serde_derive;

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
        // TODO ask peter why i have to do &frequency here.
        // Aren't i32s Copy? Shouldn't that mean that they're fine to pass around?
        if seen_frequencies.contains(&frequency) {
            return frequency;
        } else {
            seen_frequencies.insert(frequency);
        }
    }

    -1
}

fn letter_frequencies(x: &str) -> HashMap<char, i32> {
    let mut ret = HashMap::new();

    for character in x.chars() {
        let count = ret.entry(character).or_insert(0);
        *count += 1;
    }

    ret
}

// To make sure you didn't miss any, you scan the likely candidate boxes again,
// counting the number that have an ID containing exactly two of any letter and
// then separately counting those with exactly three of any letter. You can multiply
// those two counts together to get a rudimentary checksum and compare it
// to what your device predicts.
fn two_a() -> i32 {
    let contents = fs::read_to_string("src/inputs/2.txt").unwrap();

    let mut num_with_a_letter_that_appears_twice = 0;
    let mut num_with_a_letter_that_appears_thrice = 0;

    for letter_freq_map in contents.lines().map(letter_frequencies) {
        if letter_freq_map.values().into_iter().any(|&x| x == 2) {
            num_with_a_letter_that_appears_twice += 1;
        }
        if letter_freq_map.values().into_iter().any(|&x| x == 3) {
            num_with_a_letter_that_appears_thrice += 1;
        }
    }

    num_with_a_letter_that_appears_twice * num_with_a_letter_that_appears_thrice
}

fn differing_character_positions(x: &str, y: &str) -> Vec<usize> {
    x.chars()
        .enumerate()
        .filter(|(i, character)| y.chars().nth(*i).unwrap() != *character)
        .map(|(i, _)| i)
        .collect()
}

// The boxes will have IDs which differ by exactly one character at the same position in both strings.
// What letters are common between the two correct box IDs?
fn two_b() -> String {
    let contents = fs::read_to_string("src/inputs/2.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let (box_a, box_b) = lines
        .iter()
        .combinations(2)
        .map(|pair| (pair[0], pair[1]))
        .find(|(box_a, box_b)| differing_character_positions(box_a, box_b).iter().count() == 1)
        .unwrap();

    let differing_index = differing_character_positions(box_a, box_b)[0];

    let mut ret = String::new();
    for (i, character) in box_a.chars().enumerate() {
        if i != differing_index {
            ret.push(character);
        }
    }

    ret
}

#[derive(Deserialize, Debug, PartialEq)]
struct Claim {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Claim {
    fn new(claim_str: &str) -> Claim {
        let (id, x, y, width, height) = scan!("#{} @ {},{}: {}x{}" <- claim_str).unwrap();
        Claim {
            id,
            x,
            y,
            width,
            height,
        }
    }
}

fn main() {
    println!("1a: {}", one_a());
    println!("1b: {}", one_b());
    println!("2a: {}", two_a());
    println!("2b: {}", two_b());
}

#[cfg(test)]
mod test {
    use super::*;

    // I don't know anything about Rust macros yet, I'm copy-pasting this from
    // https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal for now.
    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
        };
    );

    #[test]
    // A test to ensure that I don't introduce regressions when refactoring.
    fn test_solutions() {
        assert_eq!(one_a(), 439);
        assert_eq!(one_b(), 124645);
        assert_eq!(two_a(), 5368);
        assert_eq!(two_b(), "cvgywxqubnuaefmsljdrpfzyi");
    }

    #[test]
    fn test_letter_frequencies() {
        assert_eq!(
            letter_frequencies("aabbccccd"),
            map! { 'a' => 2, 'b' => 2, 'c' => 4, 'd' => 1}
        );

        assert_eq!(
            letter_frequencies("abcabcaa"),
            map! {'a' => 4, 'b' => 2, 'c' => 2}
        );

        assert_eq!(letter_frequencies(""), HashMap::new());
    }

    #[test]
    fn test_differing_character_positions() {
        assert_eq!(differing_character_positions("abcde", "axcye"), vec![1, 3]);
        assert_eq!(differing_character_positions("fghij", "fguij"), vec![2]);
    }

    #[test]
    fn test_claim_new() {
        let input = "#123 @ 3,2: 5x4";
        assert_eq!(
            Claim::new(input),
            Claim {
                id: 123,
                x: 3,
                y: 2,
                width: 5,
                height: 4
            }
        );
    }

}
