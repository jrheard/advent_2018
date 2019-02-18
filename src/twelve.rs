use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn parse_initial_state(input: &str) -> Vec<bool> {
    // Slap 10 empty pots on either side of the initial state to handle spillover as the generations proceed.
    let mut result = vec![false; 10];

    let input = input.replace("initial state: ", "");
    result.append(&mut input.chars().map(|c| c == '#').collect());

    result.append(&mut vec![false; 10]);

    result
}

#[derive(PartialEq, Debug)]
struct GenerationRule {
    pattern: Vec<bool>,
    result: bool,
}

impl GenerationRule {
    fn new(input: &str) -> Self {
        lazy_static! {
            static ref re: Regex =
                Regex::new(r"(?P<pattern>[\.#]{5}) => (?P<result>[.#])").unwrap();
        }

        let caps = re.captures(input).unwrap();

        GenerationRule {
            pattern: caps
                .name("pattern")
                .unwrap()
                .as_str()
                .chars()
                .map(|c| c == '#')
                .collect(),
            result: caps.name("result").unwrap().as_str() == "#",
        }
    }
}

pub fn twelve_a() -> u32 {
    let contents = fs::read_to_string("src/inputs/12.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let initial_state = parse_initial_state(lines[0]);
    let rules: Vec<GenerationRule> = lines
        .iter()
        .skip(2)
        .map(|line| GenerationRule::new(line))
        .collect();

    5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {}

    #[test]
    fn test_parse_initial_state() {
        let mut expected = vec![false; 10];
        expected.append(&mut vec![
            true, false, false, true, true, false, false, false, false,
        ]);
        expected.append(&mut vec![false; 10]);

        assert_eq!(parse_initial_state("initial state: #..##...."), expected);
    }

    #[test]
    fn test_generation_rule_new() {
        assert_eq!(
            GenerationRule::new("..#.. => ."),
            GenerationRule {
                pattern: vec![false, false, true, false, false],
                result: false
            }
        );

        assert_eq!(
            GenerationRule::new(".##.# => #"),
            GenerationRule {
                pattern: vec![false, true, true, false, true],
                result: true
            }
        );
    }
}
