use std::char;

fn ten_recipes_after(num_recipes: usize) -> String {
    let mut scores = vec![3, 7];
    let mut elf_1_index = 0;
    let mut elf_2_index = 1;

    while scores.len() < num_recipes + 10 {
        let mut new_recipe = scores[elf_1_index] + scores[elf_2_index];
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
        scores.append(&mut score_digits);

        elf_1_index += 1 + scores[elf_1_index];
        elf_1_index %= scores.len();

        elf_2_index += 1 + scores[elf_2_index];
        elf_2_index %= scores.len();
    }

    let mut ret = String::new();

    for &score in scores.iter().skip(num_recipes).take(10) {
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
