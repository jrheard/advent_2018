use std::fs;
use std::sync::mpsc;
use std::thread;

/// The polymer is formed by smaller units which, when triggered, react with each other such that
/// two adjacent units of the same type and opposite polarity are destroyed. Units' types are
/// represented by letters; units' polarity is represented by capitalization. For instance, r and R
/// are units with the same type but opposite polarity, whereas r and s are entirely different types and do not react.
///
/// In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.

/// Returns true if `a` is lowercase, `b` is uppercase, and both are the same letter.
fn polymer_chars_react_one_way_check(a: char, b: char) -> bool {
    // XXX jrheard see if to_uppercase() is slow
    // it probably is!
    // maybe generate an array of lowercase and uppercase letters, one array per category
    a.is_lowercase() && a.to_uppercase().nth(0).unwrap() == b
}

/// Turns "abBA" into "aA".
fn react_polymer_one_step(polymer: &str) -> String {
    let mut ret = String::new();
    let mut prev_char = ' ';

    // TODO kill polymer_chars_react_one_way_check and just inline it?
    // or make it a closure that has access to our two arrays?

    // TODO consider just doing some math on the character's byte
    // so if it's lowercase 'f', we subtract lowercase 'a''s byte value from it
    // and now we have our index into the upper/lowercase arrays

    for character in polymer.chars() {
        let should_destroy = polymer_chars_react_one_way_check(prev_char, character)
            || polymer_chars_react_one_way_check(character, prev_char);
        if should_destroy {
            ret.pop();
            prev_char = ' ';
        } else {
            ret.push(character);
            prev_char = character;
        }
    }

    ret
}

/// Turns "cabBA" into "c".
fn react_polymer(polymer: &str) -> String {
    let mut polymer = polymer.to_string();

    loop {
        let reacted_polymer = react_polymer_one_step(&polymer[..]);
        if polymer == reacted_polymer {
            break;
        } else {
            polymer = reacted_polymer;
        }
    }

    polymer
}

/// How many units remain after fully reacting the polymer you scanned?
pub fn five_a() -> usize {
    let contents = fs::read_to_string("src/inputs/5.txt").unwrap();
    react_polymer(contents.trim()).len()
}

fn string_without_char(string: &str, character: char) -> String {
    let char_uppercase = character.to_uppercase().nth(0).unwrap();

    string
        .chars()
        .filter(|&char| char != character && char != char_uppercase)
        .collect::<String>()
}

/// One of the unit types is causing problems; it's preventing the polymer from
/// collapsing as much as it should. Your goal is to figure out which unit type
/// is causing the most problems, remove all instances of it (regardless of polarity),
/// fully react the remaining polymer, and measure its length.
pub fn five_b() -> usize {
    let contents = fs::read_to_string("src/inputs/5.txt").unwrap();
    let contents = contents.trim();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let (tx, rx) = mpsc::channel();

    for character in alphabet.chars() {
        let tx = mpsc::Sender::clone(&tx);
        let polymer = string_without_char(contents, character);
        thread::spawn(move || {
            tx.send(react_polymer(polymer.as_str())).unwrap();
        });
    }

    let mut smallest_length = std::usize::MAX;

    for _ in 0..alphabet.len() {
        let reacted_polymer = rx.recv().unwrap();
        if reacted_polymer.len() < smallest_length {
            smallest_length = reacted_polymer.len();
        }
    }

    smallest_length
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    // A test to ensure that I don't introduce regressions when refactoring.
    fn test_solutions() {
        assert_eq!(five_a(), 9900);
        assert_eq!(five_b(), 4992);
    }

    #[test]
    fn test_react_polymer() {
        assert_eq!(react_polymer("abBAacIiCdEQseztTi"), "adEQsezi");
    }
}
