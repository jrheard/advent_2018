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
        .filter(|satisfied_indexes| satisfied_indexes.len() >= 3)
        .count()
}

struct MappingComputationData {
    commitments: HashMap<usize, usize>,
    possibilities: HashMap<usize, HashSet<usize>>,
}

fn find_mapping(data: &mut MappingComputationData) -> bool {
    if data.commitments.len() == 16 {
        // Base case 1: we've successfully filled out `commitments`!
        return true;
    }

    let commitment_keys: HashSet<usize> = HashSet::from_iter(data.commitments.keys().cloned());
    let possibilities_keys: HashSet<usize> = HashSet::from_iter(data.possibilities.keys().cloned());

    for i in 0..16 {
        if !commitment_keys.contains(&i) && !possibilities_keys.contains(&i) {
            // Base case 2: An opcode has become orphaned - it isn't included in `commitments`,
            // and has no entries in `possibilities`. Return false to indicate that this set of commitments is invalid.
            return false;
        }
    }

    // Find the opcode that has the smallest number of possibilities.
    let opcode = data
        .possibilities
        .iter()
        .sorted_by_key(|(_, possible_indexes)| possible_indexes.len())
        .map(|(opcode, _)| opcode)
        .cloned()
        .nth(0)
        .unwrap();

    let possibilities_for_opcode = data.possibilities[&opcode].clone();

    for possible_operation_index in possibilities_for_opcode {
        // Record a mapping from opcode -> posible_operation_index in commitments.
        data.commitments.insert(opcode, possible_operation_index);

        // Since we've commited to `possible_operation_index` for this opcode,
        // remove it from all other possibile-indexes hashsets in `possibilities`.
        let mut affected_opcodes = vec![];
        for (opcode, v) in data.possibilities.iter_mut() {
            let did_remove = v.remove(&possible_operation_index);
            if did_remove {
                affected_opcodes.push(opcode.clone());
            }
        }

        // Get rid of entries in `possibilities` whose value is an empty hashset.
        data.possibilities.retain(|_, v| !v.is_empty());

        if find_mapping(data) {
            // This commitment ended up working out! We're done!
            return true;
        }

        // If that didn't work, this commitment was a failure.
        // Back it out and try again with the next possible operation index.

        data.commitments.remove(&opcode);

        for affected_opcode in affected_opcodes {
            data.possibilities
                .entry(affected_opcode)
                .or_insert(HashSet::new())
                .insert(possible_operation_index);
        }
    }

    false
}

fn compute_opcode_to_operation_mapping(samples: &[Sample]) -> [u8; 16] {
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

    let mut data = MappingComputationData {
        commitments: HashMap::new(),
        possibilities,
    };

    find_mapping(&mut data);

    assert_eq!(data.commitments.len(), 16);
    assert_eq!(data.commitments.values().unique().count(), 16);

    let mut ret = [0; 16];

    for i in 0..16 {
        ret[i] = data.commitments[&i] as u8;
    }

    ret
}

/// Using the samples you collected, work out the number of each opcode and execute the test program
/// (the second section of your puzzle input). What value is contained in register 0 after executing the test program?
pub fn sixteen_b() -> u8 {
    let samples = parse_input();

    // An array whose indices represent the opcodes 0-15,
    // and whose values represent the possible Operations for those opcodes.
    let mut opcode_candidates = [[true; 16]; 16];

    for sample in samples {
        let satisfied_operation_indexes = test_sample(&sample);
        for index in 0..16 {
            if !satisfied_operation_indexes.contains(&index) {
                opcode_candidates[sample.instruction[0]][index] = false;
            }
        }
    }

    dbg!(opcode_candidates);

    let mut opcode_to_operation_index = [std::usize::MAX; 16];

    for (i, candidates) in opcode_candidates.iter().enumerate() {
        for (operation_index, &value) in candidates.iter().enumerate() {
            if value {
                println!("opcode {} maps to operation {}", i, operation_index);
                opcode_to_operation_index[i] = operation_index;
            }
        }
    }

    // 1. identify mapping from opcode num to operation
    // 2. update parse fn to handle second part of file
    // 3. run program
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
