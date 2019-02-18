use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn parse_input() -> (Vec<bool>, Vec<GenerationRule>) {
    let contents = fs::read_to_string("src/inputs/12.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    (
        lines[0]
            .replace("initial state: ", "")
            .chars()
            .map(|c| c == '#')
            .collect(),
        lines
            .iter()
            .skip(2)
            .map(|line| GenerationRule::new(line))
            .collect(),
    )
}

#[derive(PartialEq, Debug)]
pub struct GenerationRule {
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

// TODO: separate a and b functions
// TODO: write test for b

mod cave {
    use super::*;

    const PLANT_BUFFER_LENGTH: usize = 1000;

    pub struct Cave {
        plants: Vec<bool>,
        rules: Vec<GenerationRule>,
        first_plant_index: usize,
        last_plant_index: usize,
    }

    impl Cave {
        pub fn new(initial_state: Vec<bool>, rules: Vec<GenerationRule>) -> Self {
            let mut buf = vec![false; PLANT_BUFFER_LENGTH];

            for i in 0..initial_state.len() {
                buf[PLANT_BUFFER_LENGTH / 2 + i] = initial_state[i];
            }

            Cave {
                plants: buf,
                rules,
                first_plant_index: PLANT_BUFFER_LENGTH / 2,
                last_plant_index: PLANT_BUFFER_LENGTH / 2 + initial_state.len(),
            }
        }

        pub fn tick_generation(&mut self) -> i32 {
            let mut new_generation = vec![];

            let pots = self
                .plants
                .iter()
                .skip(self.first_plant_index - 3)
                // xxxx 9?
                .take((self.last_plant_index - self.first_plant_index) + 9)
                .cloned()
                .collect::<Vec<bool>>();

            for window in pots.windows(5) {
                for rule in &self.rules {
                    if rule.pattern == window {
                        new_generation.push(rule.result);
                        break;
                    }
                }
            }

            let original_first_plant_index = self.first_plant_index;

            for (i, &value) in new_generation.iter().enumerate() {
                let translated_index = i + original_first_plant_index - 3 + 2;

                if value {
                    if translated_index < self.first_plant_index {
                        self.first_plant_index = translated_index;
                    } else if translated_index > self.last_plant_index {
                        self.last_plant_index = translated_index;
                    }
                }

                self.plants[translated_index] = value;
            }

            self.plants
                .iter()
                .enumerate()
                .filter(|(_, &value)| value)
                .map(|(index, _)| index as i32 - (PLANT_BUFFER_LENGTH / 2) as i32)
                .sum()
        }
    }
}

const FIFTY_BILLION: u64 = 50000000000;

pub fn twelve_a() -> i32 {
    let (initial_state, rules) = parse_input();

    let mut plant_cave = cave::Cave::new(initial_state, rules);

    for _ in 0..19 {
        plant_cave.tick_generation();
    }

    plant_cave.tick_generation()
}

pub fn twelve_b() -> u64 {
    let (initial_state, rules) = parse_input();

    let mut plant_cave = cave::Cave::new(initial_state, rules);

    let mut previous_sum = 0;
    let mut previous_sum_increase = 0;
    let mut num_times_saw_same_sum_increase_in_a_row = 0;

    for i in 0..FIFTY_BILLION {
        let sum = plant_cave.tick_generation();

        if (sum - previous_sum) == previous_sum_increase {
            num_times_saw_same_sum_increase_in_a_row += 1;

            if num_times_saw_same_sum_increase_in_a_row > 10 {
                return previous_sum as u64
                    + ((FIFTY_BILLION - i) * (sum as u64 - previous_sum as u64));
            }
        } else {
            num_times_saw_same_sum_increase_in_a_row = 0;
        }
        previous_sum_increase = sum - previous_sum;
        previous_sum = sum;
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(twelve_a(), 3276);
        assert_eq!(twelve_b(), 3750000001113);
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
