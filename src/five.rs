use std::fs;

use rayon::prelude::*;

/// The polymer is formed by smaller units which, when triggered, react with each other such that
/// two adjacent units of the same type and opposite polarity are destroyed. Units' types are
/// represented by letters; units' polarity is represented by capitalization. For instance, r and R
/// are units with the same type but opposite polarity, whereas r and s are entirely different types and do not react.
///
/// In abBA, bB destroys itself, leaving aA. As above, this then destroys itself, leaving nothing.

/// Turns "abBA" into "aA".
fn react_polymer_one_step(polymer: &[u8]) -> Vec<u8> {
    // Got the idea to use a Vec::<u8> here from forrestthewoods.
    let mut ret = Vec::<u8>::with_capacity(polymer.len());
    let mut prev_char = b' ';

    for &character in polymer {
        if (prev_char.is_ascii_lowercase() && prev_char.to_ascii_uppercase() == character)
            || (character.is_ascii_lowercase() && character.to_ascii_uppercase() == prev_char)
        {
            ret.pop();
            prev_char = b' ';
        } else {
            ret.push(character as u8);
            prev_char = character;
        }
    }

    ret
}

/// Turns "cabBA" into "c".
fn react_polymer(polymer: Vec<u8>) -> Vec<u8> {
    let mut polymer = polymer;
    loop {
        let reacted_polymer = react_polymer_one_step(&polymer);
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
    react_polymer(contents.trim().as_bytes().to_vec()).len()
}

fn buf_without_char(buf: &[u8], to_remove: u8) -> Vec<u8> {
    let to_remove_uppercase = to_remove.to_ascii_uppercase();

    buf.iter()
        .filter(|&&character| character != to_remove && character != to_remove_uppercase)
        .cloned()
        .collect()
}

/// One of the unit types is causing problems; it's preventing the polymer from
/// collapsing as much as it should. Your goal is to figure out which unit type
/// is causing the most problems, remove all instances of it (regardless of polarity),
/// fully react the remaining polymer, and measure its length.
pub fn five_b() -> usize {
    let contents = fs::read_to_string("src/inputs/5.txt").unwrap();
    let contents = contents.trim().as_bytes().to_vec();

    "abcdefghijklmnopqrstuvwxyz"
        .par_chars()
        .map(|character| react_polymer(buf_without_char(&contents, character as u8)).len())
        .min()
        .unwrap()
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
        assert_eq!(react_polymer(b"abBAacIiCdEQseztTi".to_vec()), b"adEQsezi");
    }
}
