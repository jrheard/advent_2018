use std::fs;

use itertools::Itertools;
use serde_scan::scan;

// implement all sixteen opcodes

#[derive(Debug, PartialEq)]
struct Sample {
    before: [u8; 4],
    instruction: [usize; 4],
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

type Operation = Fn(&mut [u8; 4], usize, usize, usize);

fn get_operations() -> Vec<Box<Operation>> {
    vec![
        // "addr (add register) stores into register C the result of adding register A and register B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] + registers[b]),
        // "addi (add immediate) stores into register C the result of adding register A and value B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] + b as u8),
        // "mulr (multiply register) stores into register C the result of multiplying register A and register B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] * registers[b]),
        // "muli (multiply immediate) stores into register C the result of multiplying register A and value B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] * b as u8),
        // "banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] & registers[b]),
        // "bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] & b as u8),
        // "borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] | registers[b]),
        // "bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] | b as u8),
        // "setr (set register) copies the contents of register A into register C. (Input B is ignored.)"
        Box::new(|registers, a, _, c| registers[c] = registers[a]),
        // "seti (set immediate) stores value A into register C. (Input B is ignored.)"
        Box::new(|registers, a, _, c| registers[c] = a as u8),
        // "gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if a as u8 > registers[b] { 1 } else { 0 }),
        // "gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if registers[a] > b as u8 { 1 } else { 0 }),
        // "gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if registers[a] > registers[b] { 1 } else { 0 }),
        // "eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if a as u8 == registers[b] { 1 } else { 0 }),
        // "eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if registers[a] == b as u8 { 1 } else { 0 }),
        // "eqrr (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if registers[a] == registers[b] { 1 } else { 0 }),
    ]
}

/// Returns a Vec containing the indexes of the Operations whose behavior satisfies this Sample.
fn test_sample(sample: &Sample) -> Vec<usize> {
    let mut ret = vec![];

    let (a, b, c) = (sample.instruction[1], sample.instruction[2], sample.instruction[3]);

    for (i, operation) in get_operations().iter().enumerate() {
        let mut output = sample.before;

        operation(&mut output, a, b, c);

        if output == sample.after {
            ret.push(i);
        }
    }

    ret
}

/// Ignoring the opcode numbers, how many samples in your puzzle input behave like three or more opcodes?
pub fn sixteen_a() -> usize {
    let samples = parse_input();

    samples
        .iter()
        .map(|sample| test_sample(sample))
        .filter(|satisfied_indexes| satisfied_indexes.len() > 3)
        .count()
}

/// Using the samples you collected, work out the number of each opcode and execute the test program
/// (the second section of your puzzle input). What value is contained in register 0 after executing the test program?
pub fn sixteen_b() -> u8 {
    5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(sixteen_a(), 588);
    }

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
