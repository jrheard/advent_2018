#![cfg_attr(feature = "cargo-clippy", allow(clippy::needless_range_loop))]
use std::fs;
use std::iter::FromIterator;

use hashbrown::HashMap;
use hashbrown::HashSet;
use itertools::Itertools;
use serde_scan::scan;

// implement all sixteen opcodes

#[derive(Debug, PartialEq)]
struct Sample {
    before: [usize; 4],
    instruction: [usize; 4],
    after: [usize; 4],
}

fn parse_input() -> (Vec<Sample>, Vec<[usize; 4]>) {
    let contents = fs::read_to_string("src/inputs/16.txt").unwrap();

    let mut first_half = vec![];

    for mut chunk in &contents.lines().chunks(4) {
        let before = chunk.next().unwrap();
        if !before.starts_with("Before") {
            // We've reached the second part of the puzzle input.
            break;
        }

        let (b_a, b_b, b_c, b_d) = scan!("Before: [{}, {}, {}, {}]" <- before).unwrap();
        let instruction = chunk.next().unwrap();
        let (i_a, i_b, i_c, i_d) = scan!("{} {} {} {}" <- instruction).unwrap();
        let after = chunk.next().unwrap();
        let (a_a, a_b, a_c, a_d) = scan!("After: [{}, {}, {}, {}]" <- after).unwrap();

        first_half.push(Sample {
            before: [b_a, b_b, b_c, b_d],
            instruction: [i_a, i_b, i_c, i_d],
            after: [a_a, a_b, a_c, a_d],
        });
    }

    let mut second_half_index = 0;
    let mut newlines_seen_in_a_row = 0;
    let lines = contents.lines().collect::<Vec<&str>>();

    for (i, &line) in lines.iter().enumerate() {
        if line == "" {
            newlines_seen_in_a_row += 1;
        } else {
            newlines_seen_in_a_row = 0;
        }

        if newlines_seen_in_a_row == 3 {
            second_half_index = i + 1;
            break;
        }
    }

    let mut second_half = vec![];

    for &line in &lines[second_half_index..] {
        second_half.push(scan!("{} {} {} {}" <- line).unwrap());
    }

    (first_half, second_half)
}

type Operation = Fn(&mut [usize; 4], usize, usize, usize);

fn get_operations() -> Vec<Box<Operation>> {
    vec![
        // "addr (add register) stores into register C the result of adding register A and register B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] + registers[b]),
        // "addi (add immediate) stores into register C the result of adding register A and value B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] + b),
        // "mulr (multiply register) stores into register C the result of multiplying register A and register B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] * registers[b]),
        // "muli (multiply immediate) stores into register C the result of multiplying register A and value B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] * b),
        // "banr (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] & registers[b]),
        // "bani (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] & b),
        // "borr (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] | registers[b]),
        // "bori (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B."
        Box::new(|registers, a, b, c| registers[c] = registers[a] | b),
        // "setr (set register) copies the contents of register A into register C. (Input B is ignored.)"
        Box::new(|registers, a, _, c| registers[c] = registers[a]),
        // "seti (set immediate) stores value A into register C. (Input B is ignored.)"
        Box::new(|registers, a, _, c| registers[c] = a),
        // "gtir (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if a > registers[b] { 1 } else { 0 }),
        // "gtri (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if registers[a] > b { 1 } else { 0 }),
        // "gtrr (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if registers[a] > registers[b] { 1 } else { 0 }),
        // "eqir (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if a == registers[b] { 1 } else { 0 }),
        // "eqri (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0."
        Box::new(|registers, a, b, c| registers[c] = if registers[a] == b { 1 } else { 0 }),
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
    let (samples, _) = parse_input();

    samples
        .iter()
        .map(|sample| test_sample(sample))
        .filter(|satisfied_indexes| satisfied_indexes.len() >= 3)
        .count()
}

fn find_mapping(mut possibilities: HashMap<usize, HashSet<usize>>) -> HashMap<usize, usize> {
    let mut mapping = HashMap::new();

    while mapping.len() < 16 {
        // At every step along the way, there should be an opcode that has only 1 possible operation index.
        let opcode = possibilities
            .iter()
            .filter(|(_, possible_indexes)| possible_indexes.len() == 1)
            .map(|(opcode, _)| opcode)
            .nth(0)
            .unwrap();

        let possibilities_for_opcode = possibilities[&opcode].clone();
        let index = possibilities_for_opcode.iter().nth(0).unwrap();

        // We've successfully found the index for this opcode!
        mapping.insert(*opcode, *index);

        // Since we've commited to `index` for this opcode,
        // remove it from all other possibile-indexes hashsets in `possibilities`.
        for v in possibilities.values_mut() {
            v.remove(&index);
        }
    }

    assert_eq!(mapping.len(), 16);
    assert_eq!(mapping.values().unique().count(), 16);

    mapping
}

fn compute_opcode_to_operation_mapping(samples: &[Sample]) -> [usize; 16] {
    // `possibilities` is a map of opcode -> possible operation index.
    // It'll have entries like {5: #{2, 4, 11}}.
    let mut possibilities = HashMap::new();

    for i in 0..16 {
        // All operation indexes are possible candidates until proven otherwise.
        possibilities.insert(i, HashSet::from_iter(0..16));
    }

    for sample in samples {
        let satisfied_operation_indexes = test_sample(&sample);

        for index in 0..16 {
            if !satisfied_operation_indexes.contains(&index) {
                // The operation with at this index doesn't satisfy the opcode `sample.instructions[0]`.
                // It's not a possible candidate for this opcode!
                possibilities.entry(sample.instruction[0]).and_modify(|set| {
                    set.remove(&index);
                });
            }
        }
    }

    let mapping = find_mapping(possibilities);

    let mut ret = [0; 16];

    for i in 0..16 {
        ret[i] = mapping[&i];
    }

    ret
}

/// Using the samples you collected, work out the number of each opcode and execute the test program
/// (the second section of your puzzle input). What value is contained in register 0 after executing the test program?
pub fn sixteen_b() -> usize {
    let (samples, program) = parse_input();
    let opcodes_to_operation_indexes = compute_opcode_to_operation_mapping(&samples);
    let operations = get_operations();
    let mut registers = [0, 0, 0, 0];

    for line in program {
        let (a, b, c, d) = (line[0], line[1], line[2], line[3]);
        let operation = &operations[opcodes_to_operation_indexes[a]];
        operation(&mut registers, b, c, d);
    }

    registers[0]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(sixteen_a(), 588);
        assert_eq!(sixteen_b(), 627);
    }

    #[test]
    fn test_parse_input() {
        let (samples, _) = parse_input();

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
