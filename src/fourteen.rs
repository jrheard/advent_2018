use std::char;

struct ElfCooks {
    scores: Vec<u8>,
    elf_1_index: usize,
    elf_2_index: usize,
}

impl ElfCooks {
    fn new() -> Self {
        ElfCooks {
            scores: vec![3, 7],
            elf_1_index: 0,
            elf_2_index: 1,
        }
    }

    fn tick(&mut self) -> Vec<u8> {
        let mut new_recipe = self.scores[self.elf_1_index] + self.scores[self.elf_2_index];
        let mut score_digits = vec![];

        if new_recipe == 0 {
            score_digits.push(0);
        } else {
            while new_recipe > 0 {
                score_digits.push(new_recipe % 10);
                new_recipe /= 10;
            }
        }

        score_digits.reverse();

        let ret = score_digits.clone();

        self.scores.append(&mut score_digits);

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
    ten_recipes_after(209231)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(fourteen_a(), "6126491027");
    }

    #[test]
    fn test_examples_from_writeup() {
        assert_eq!(ten_recipes_after(5), "0124515891".to_string());
        assert_eq!(ten_recipes_after(9), "5158916779".to_string());
        assert_eq!(ten_recipes_after(18), "9251071085".to_string());
        assert_eq!(ten_recipes_after(2018), "5941429882".to_string());
    }
}
