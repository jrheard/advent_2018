use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn parse_initial_state(input: &str) -> Vec<bool> {
    input
        .replace("initial state: ", "")
        .chars()
        .map(|c| c == '#')
        .collect()
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

const BUF_WIDTH: usize = 1000000;

pub fn twelve_a() -> i32 {
    let contents = fs::read_to_string("src/inputs/12.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let rules: Vec<GenerationRule> = lines
        .iter()
        .skip(2)
        .map(|line| GenerationRule::new(line))
        .collect();

    let initial_state = parse_initial_state(lines[0]);

    let mut buf = vec![false; BUF_WIDTH];

    for i in 0..initial_state.len() {
        buf[BUF_WIDTH / 2 + i] = initial_state[i];
    }

    let mut first_plant_index = BUF_WIDTH / 2;
    let mut last_plant_index = BUF_WIDTH / 2 + initial_state.len();

    for _ in 0..20 {
        let mut new_generation = vec![];

        let pots = buf
            .iter()
            .skip(first_plant_index - 3)
            .take(last_plant_index + 3)
            .cloned()
            .collect::<Vec<bool>>();

        for window in pots.windows(5) {
            for rule in &rules {
                if rule.pattern == window {
                    new_generation.push(rule.result);
                    break;
                }
            }
        }

        for (i, &value) in new_generation.iter().enumerate() {
            let translated_index = i + first_plant_index - 3;

            if value {
                if translated_index < first_plant_index {
                    first_plant_index = translated_index;
                } else if translated_index > last_plant_index {
                    last_plant_index = translated_index;
                }
            }

            buf[translated_index] = value;
        }
    }

    let indexes = buf
        .iter()
        .enumerate()
        .filter(|(_, &value)| value)
        .map(|(index, _)| index as i32 - (BUF_WIDTH / 2) as i32)
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
        assert_eq!(
            parse_initial_state("initial state: #..##...."),
            vec![true, false, false, true, true, false, false, false, false,]
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
