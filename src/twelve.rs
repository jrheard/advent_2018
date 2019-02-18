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

pub fn twelve_a() -> u64 {
    let contents = fs::read_to_string("src/inputs/12.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let rules: Vec<GenerationRule> = lines
        .iter()
        .skip(2)
        .map(|line| GenerationRule::new(line))
        .collect();

    let initial_state = parse_initial_state(lines[0]);

    let initial_state_string = initial_state
        .iter()
        .map(|&x| if x { '#' } else { '.' })
        .collect::<String>();
    //dbg!(initial_state_string);

    let mut buf = vec![false; BUF_WIDTH];

    for i in 0..initial_state.len() {
        buf[BUF_WIDTH / 2 + i] = initial_state[i];
    }

    /*
    println!(
        "placed initial buffer from {} to {}",
        BUF_WIDTH / 2,
        BUF_WIDTH / 2 + initial_state.len() - 1
    );
    */

    let mut first_plant_index = BUF_WIDTH / 2;
    let mut last_plant_index = BUF_WIDTH / 2 + initial_state.len();

    let mut previous_sum = 0;
    let mut previous_sum_increase = 0;
    let mut num_times_saw_same_sum_increase_in_a_row = 0;

    for i in 0u64..50000000000 {
        if i % 1000 == 0 {
            dbg!(i);
        }

        //for _ in 0..20 {
        //dbg!(first_plant_index);
        //dbg!(last_plant_index);
        let mut new_generation = vec![];

        let pots = buf
            .iter()
            .skip(first_plant_index - 3)
            .take((last_plant_index - first_plant_index) + 9)
            .cloned()
            .collect::<Vec<bool>>();

        //dbg!(pots.len());

        for window in pots.windows(5) {
            let mut has_plant = false;
            for rule in &rules {
                /* PROD CODE
                if rule.pattern == window {
                    new_generation.push(rule.result);
                    break;
                }
                */
                if rule.pattern == window {
                    has_plant = rule.result;
                    break;
                }
            }
            new_generation.push(has_plant);
        }

        /*
        let generation_string = new_generation
            .iter()
            .map(|&x| if x { '#' } else { '.' })
            .collect::<String>();
        dbg!(generation_string);
        */

        let original_first_plant_index = first_plant_index;

        for (i, &value) in new_generation.iter().enumerate() {
            let translated_index = i + original_first_plant_index - 3 + 2;

            if value {
                if translated_index < first_plant_index {
                    first_plant_index = translated_index;
                } else if translated_index > last_plant_index {
                    last_plant_index = translated_index;
                }
            }

            buf[translated_index] = value;
        }

        /*
        let generation_string = buf
            .iter()
            .skip(BUF_WIDTH / 2 - 15)
            .take(60)
            .map(|&x| if x { '#' } else { '.' })
            .collect::<String>();
        dbg!(generation_string);*/

        let indexes = buf
            .iter()
            .enumerate()
            .filter(|(_, &value)| value)
            .map(|(index, _)| index as i32 - (BUF_WIDTH / 2) as i32)
            .collect::<Vec<i32>>();

        let sum: i32 = indexes.iter().sum();

        if (sum - previous_sum) == previous_sum_increase {
            num_times_saw_same_sum_increase_in_a_row += 1;

            if num_times_saw_same_sum_increase_in_a_row > 10 {
                return previous_sum as u64
                    + ((50000000000 - i) * (sum as u64 - previous_sum as u64));
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
