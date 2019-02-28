use std::fs;

use itertools::Itertools;
use serde_scan::scan;

// implement all sixteen opcodes

#[derive(Debug, PartialEq)]
struct Sample {
    before: [u8; 4],
    instruction: [u8; 4],
    after: [u8; 4],
}

fn parse_input() -> Vec<Sample> {
    let contents = fs::read_to_string("src/inputs/16.txt").unwrap();

    let mut ret = vec![];

    for mut chunk in &contents.lines().chunks(4) {
        let before = chunk.next().unwrap();
        if !before.starts_with("Before") {
            // "The manual also includes a small test program (the second section of your puzzle input) - you can ignore it for now.""
            break;
        }

        let (b_a, b_b, b_c, b_d) = scan!("Before: [{}, {}, {}, {}]" <- before).unwrap();
        let instruction = chunk.next().unwrap();
        let (i_a, i_b, i_c, i_d) = scan!("{} {} {} {}" <- instruction).unwrap();
        let after = chunk.next().unwrap();
        let (a_a, a_b, a_c, a_d) = scan!("After: [{}, {}, {}, {}]" <- after).unwrap();

        ret.push(Sample {
            before: [b_a, b_b, b_c, b_d],
            instruction: [i_a, i_b, i_c, i_d],
            after: [a_a, a_b, a_c, a_d],
        });
    }

    ret
}

pub fn sixteen_a() -> usize {
    5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {}

    #[test]
    fn test_parse_input() {
        let samples = parse_input();

        assert_eq!(samples.len(), 776);

        assert_eq!(
            samples[2],
            Sample {
                before: [2, 0, 1, 0],
                instruction: [0, 2, 1, 3],
                after: [2, 0, 1, 1]
            }
        );
    }
}
