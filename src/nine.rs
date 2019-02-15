//use std::fs;

fn next_marble_position(current_marble_index: usize, circle: &[u32]) -> usize {
    let len = circle.len();

    if len == 1 {
        1
    } else {
        let new_index = current_marble_index + 2;

        if new_index > len {
            new_index % len
        } else {
            new_index % (len + 1)
        }
    }
}

pub fn nine_a() -> u32 {
    let num_players = 10;
    let last_marble = 25;
    //let last_marble = 1618;

    let mut current_player = 0;
    let mut current_marble_index: usize = 0;
    let mut scores = vec![0 as u32; num_players];

    let mut circle = vec![0];

    for i in 1..=last_marble {
        if i % 23 == 0 {
            scores[current_player] += i;

            dbg!(current_marble_index);
            let mut index_to_remove = (current_marble_index as i32 - 7) % circle.len() as i32;
            if index_to_remove < 0 {
                // xxxxx
                index_to_remove += circle.len() as i32 - 1;
            }

            current_marble_index = (index_to_remove as usize + 1) % circle.len();
            dbg!(index_to_remove);
            dbg!(current_marble_index);
            scores[current_player] += circle.remove(index_to_remove as usize);
        } else {
            current_marble_index = next_marble_position(current_marble_index, &circle);
            circle.insert(current_marble_index, i);
        }

        current_player = (current_player + 1) % num_players;
    }

    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {}

    #[test]
    fn test_next_marble_position() {
        assert_eq!(next_marble_position(0, &[0]), 1);
        assert_eq!(next_marble_position(1, &[0, 1]), 1);
        assert_eq!(next_marble_position(1, &[0, 2, 1]), 3);
        assert_eq!(next_marble_position(3, &[0, 2, 1, 3]), 1);
        assert_eq!(next_marble_position(1, &[0, 4, 2, 1, 3]), 3);
        assert_eq!(next_marble_position(3, &[0, 4, 2, 5, 1, 3]), 5);
        assert_eq!(next_marble_position(5, &[0, 4, 2, 5, 1, 6, 3]), 7);
        assert_eq!(next_marble_position(7, &[0, 4, 2, 5, 1, 6, 3, 7]), 1);
    }
}
