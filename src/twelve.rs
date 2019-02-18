use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn parse_initial_state(input: &str) -> Vec<bool> {
    input.chars().map(|c| c == '#').collect()
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
    5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {}

    #[test]
    fn test_parse_initial_state() {
        assert_eq!(
            parse_initial_state("#..##...."),
            [true, false, false, true, true, false, false, false, false]
        );
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
