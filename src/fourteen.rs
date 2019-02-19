use std::char;
use std::collections::VecDeque;

struct ElfCooks {
    /// "The Elves are trying to come up with the ultimate hot chocolate recipe;
    /// they're even maintaining a scoreboard which tracks the quality score (0-9) of each recipe."
    scores: Vec<u8>,
    elf_1_index: usize,
    elf_2_index: usize,
}

const SENTINEL_DIGIT_VALUE: u8 = 99;

impl ElfCooks {
    fn new() -> Self {
        ElfCooks {
            // "Only two recipes are on the board: the first recipe got a score of 3, the second, 7.""
            scores: vec![3, 7],

            // "Each of the two Elves has a current recipe: the first Elf starts with the first recipe,
            // and the second Elf starts with the second recipe."
            elf_1_index: 0,
            elf_2_index: 1,
        }
    }

    fn tick(&mut self, ret: &mut Vec<u8>) {
        // "To create new recipes, the two Elves combine their current recipes. This creates
        // new recipes from the digits of the sum of the current recipes' scores."
        let mut new_recipe = self.scores[self.elf_1_index] + self.scores[self.elf_2_index];

        let mut score_digits = [SENTINEL_DIGIT_VALUE; 2];

        if new_recipe == 0 {
            score_digits[0] = 0;
        } else {
            score_digits[0] = new_recipe % 10;
            new_recipe /= 10;

            if new_recipe > 0 {
                score_digits[1] = new_recipe;
            }
        }

        if score_digits[1] != SENTINEL_DIGIT_VALUE {
            self.scores.push(score_digits[1]);
            ret.push(score_digits[1]);
        }

        if score_digits[0] != SENTINEL_DIGIT_VALUE {
            self.scores.push(score_digits[0]);
            ret.push(score_digits[0]);
        }

        // "After all new recipes are added to the scoreboard, each Elf picks a new current recipe.
        // To do this, the Elf steps forward through the scoreboard a number of recipes equal to
        // 1 plus the score of their current recipe. If they run out of recipes, they
        // loop back around to the beginning."
        self.elf_1_index += 1 + self.scores[self.elf_1_index] as usize;
        self.elf_1_index %= self.scores.len();

        self.elf_2_index += 1 + self.scores[self.elf_2_index] as usize;
        self.elf_2_index %= self.scores.len();
    }
}

fn ten_recipes_after(num_recipes: usize) -> String {
    let mut elves = ElfCooks::new();

    let mut v = vec![];
    while elves.scores.len() < num_recipes + 10 {
        elves.tick(&mut v);
        v.clear();
    }

    let mut ret = String::new();

    for &score in elves.scores.iter().skip(num_recipes).take(10) {
        ret.push(char::from_digit(score as u32, 10).unwrap());
    }

    ret
}

/// What are the scores of the ten recipes immediately after the number of recipes in your puzzle input?
pub fn fourteen_a() -> String {
    ten_recipes_after(209231)
}

/// How many recipes appear on the scoreboard to the left of the score sequence in your puzzle input?
pub fn fourteen_b() -> usize {
    let input = [2, 0, 9, 2, 3, 1];
    let input_length = input.len();

    let mut elves = ElfCooks::new();

    let mut window = VecDeque::with_capacity(21000000);
    window.push_back(3);
    window.push_back(7);

    let mut num_scores_seen = 2;

    let mut new_scores = vec![];

    loop {
        elves.tick(&mut new_scores);

        for &score in &new_scores {
            num_scores_seen += 1;

            window.push_back(score);

            if window == input {
                return num_scores_seen - window.len();
            }

            if window.len() >= input_length {
                window.pop_front();
            }
        }

        new_scores.clear();
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
