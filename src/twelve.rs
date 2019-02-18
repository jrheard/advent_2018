use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

fn parse_input() -> cave::Cave {
    let contents = fs::read_to_string("src/inputs/12.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    cave::Cave::new(
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
    /// "Someone has been trying to figure out how these plants spread to nearby pots.
    /// Based on the notes, for each generation of plants, a given pot has or does not have
    /// a plant based on whether that pot (and the two pots on either side of it) had a plant
    /// in the last generation. These are written as LLCRR => N, where L are pots to the left,
    /// C is the current pot being considered, R are the pots to the right, and N is whether
    /// the current pot will have a plant in the next generation."
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

mod cave {
    use super::*;

    const PLANT_BUFFER_LENGTH: usize = 1000;

    /// After exploring a little, you discover a long tunnel that contains a row of small pots
    /// as far as you can see to your left and right. A few of them contain plants - someone
    /// is trying to grow things in these geothermally-heated caves.
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

        /// Applies self.rules to self.plants.
        /// Returns the sum of the indexes of the pots which contain a plant in the new generation.
        pub fn tick_generation(&mut self) -> i32 {
            let mut new_generation = vec![];

            let pots = self
                .plants
                .iter()
                .skip(self.first_plant_index - 3)
                .take((self.last_plant_index - self.first_plant_index) + 7)
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

            for (i, &pot_has_plant) in new_generation.iter().enumerate() {
                // - 3 because of the offset in the `.skip()` call earlier,
                // + 2 because the actual plant whose fate is being determined
                // is the one in the middle of this window of 5 pots.
                let translated_index = i + original_first_plant_index - 3 + 2;

                if pot_has_plant {
                    if translated_index < self.first_plant_index {
                        self.first_plant_index = translated_index;
                    } else if translated_index > self.last_plant_index {
                        self.last_plant_index = translated_index;
                    }
                }

                self.plants[translated_index] = pot_has_plant;
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

/// After 20 generations, what is the sum of the numbers of all pots which contain a plant?
pub fn twelve_a() -> i32 {
    let mut plant_cave = parse_input();

    for _ in 0..19 {
        plant_cave.tick_generation();
    }

    plant_cave.tick_generation()
}

/// After fifty billion (50000000000) generations, what is the sum of the numbers of all pots which contain a plant?
pub fn twelve_b() -> u64 {
    let mut plant_cave = parse_input();

    let mut previous_sum = 0;
    let mut previous_sum_increase = 0;
    let mut num_times_saw_same_sum_increase_in_a_row = 0;

    for i in 0..FIFTY_BILLION {
        let sum = plant_cave.tick_generation();

        if (sum - previous_sum) == previous_sum_increase {
            num_times_saw_same_sum_increase_in_a_row += 1;

            if num_times_saw_same_sum_increase_in_a_row > 10 {
                // Our cave has reached a stable cycle, and we can safely predict
                // what the plant sum will look like in (fifty billion - num_elapsed) generations.
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
