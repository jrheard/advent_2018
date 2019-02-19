use std::char;
use std::collections::VecDeque;
use std::iter::FromIterator;

struct ElfCooks {
    scores: Vec<u8>,
    window: VecDeque<u8>,
    elf_1_index: usize,
    elf_2_index: usize,
}

impl ElfCooks {
    fn new() -> Self {
        ElfCooks {
            scores: vec![3, 7],
            window: VecDeque::from_iter(vec![3, 7]),
            elf_1_index: 0,
            elf_2_index: 1,
        }
    }

    fn tick(&mut self) -> Vec<u8> {
        let mut new_recipe = self.scores[self.elf_1_index] + self.scores[self.elf_2_index];
        // XXXXX let's get rid of these vector allocations
        // both this score_digits vec and the ret vec
        // xxxx what if not vecs
        let mut score_digits = [10; 5];

        if new_recipe == 0 {
            score_digits[0] = 0;
        } else {
            let mut i = 0;
            while new_recipe > 0 {
                score_digits[i] = new_recipe % 10;
                new_recipe /= 10;
                i += 1;
            }
        }

        let mut ret = vec![];

        for &digit in score_digits.iter() {
            if digit == 10 {
                break;
            }

            self.scores.push(digit);
            ret.push(digit);
        }

        self.elf_1_index += 1 + self.scores[self.elf_1_index] as usize;
        self.elf_1_index %= self.scores.len();

        self.elf_2_index += 1 + self.scores[self.elf_2_index] as usize;
        self.elf_2_index %= self.scores.len();

        ret
    }
}

fn ten_recipes_after(num_recipes: usize) -> String {
    let mut elves = ElfCooks::new();

    while elves.scores.len() < num_recipes + 10 {
        let _ = elves.tick();
    }

    let mut ret = String::new();

    for &score in elves.scores.iter().skip(num_recipes).take(10) {
        ret.push(char::from_digit(score as u32, 10).unwrap());
    }

    ret
}

pub fn fourteen_a() -> String {
    // XXXX returns wrong answer
    ten_recipes_after(209231)
}

pub fn fourteen_b() -> usize {
    return 3;
    let input = [2, 0, 9, 2, 3, 1];
    let input_length = input.len();

    let mut elves = ElfCooks::new();

    let mut window = VecDeque::new();
    window.push_back(3);
    window.push_back(7);

    let mut num_scores_seen = 2;

    loop {
        let new_scores = elves.tick();

        for score in new_scores {
            num_scores_seen += 1;

            window.push_back(score);

            if window == input {
                return num_scores_seen - window.len();
            }

            if window.len() >= input_length {
                window.pop_front();
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(fourteen_a(), "6126491027");
        assert_eq!(fourteen_b(), 20191616);
    }

    #[test]
    fn test_examples_from_writeup() {
        assert_eq!(ten_recipes_after(5), "0124515891".to_string());
        assert_eq!(ten_recipes_after(9), "5158916779".to_string());
        assert_eq!(ten_recipes_after(18), "9251071085".to_string());
        assert_eq!(ten_recipes_after(2018), "5941429882".to_string());
    }
}
