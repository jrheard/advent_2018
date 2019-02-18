use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

const NUM_PADDING_POTS: usize = 30;

fn parse_initial_state(input: &str) -> Vec<bool> {
    // Slap 15 empty pots on either side of the initial state to handle spillover as the generations proceed.
    let mut result = vec![false; NUM_PADDING_POTS];

    let input = input.replace("initial state: ", "");
    result.append(&mut input.chars().map(|c| c == '#').collect());

    result.append(&mut vec![false; NUM_PADDING_POTS]);

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

#[allow(dead_code)]
fn print_generation(pots: &Vec<bool>) {
    let generation: String = pots
        .iter()
        .map(|&pot| if pot { '#' } else { '.' })
        .collect();
    println!("{}", generation);
}

pub fn twelve_a() -> i32 {
    let contents = fs::read_to_string("src/inputs/12.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let rules: Vec<GenerationRule> = lines
        .iter()
        .skip(2)
        .map(|line| GenerationRule::new(line))
        .collect();

    let mut generation = parse_initial_state(lines[0]);

    for _ in 0..20 {
        let mut new_generation = generation.clone();

        let windows = generation.windows(5).enumerate();
        for (i, window) in windows {
            for rule in &rules {
                if rule.pattern == window {
                    new_generation[i + 2] = rule.result;
                    break;
                }
            }
        }

        generation = new_generation;
    }

    let indexes = generation
        .iter()
        .enumerate()
        .filter(|(_, &value)| value)
        .map(|(index, _)| index as i32 - NUM_PADDING_POTS as i32)
        .collect::<Vec<i32>>();

    indexes.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(twelve_a(), 3276);
    }

    #[test]
    fn test_parse_initial_state() {
        let mut expected = vec![false; NUM_PADDING_POTS];
        expected.append(&mut vec![
            true, false, false, true, true, false, false, false, false,
        ]);
        expected.append(&mut vec![false; NUM_PADDING_POTS]);

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
