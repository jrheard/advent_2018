//use std::fs;

mod game {
    use std::collections::VecDeque;
    use std::mem;

    pub struct MarbleGame {
        left: VecDeque<usize>,
        right: VecDeque<usize>,

        num_players: usize,
        current_player: usize,

        next_marble_id: usize,
        // TODO what are the semantics of current_marble?
        current_marble: usize,
    }

    impl MarbleGame {
        pub fn new(num_players: usize, last_marble: usize) -> MarbleGame {
            MarbleGame {
                left: VecDeque::with_capacity(last_marble),
                right: VecDeque::with_capacity(last_marble),
                num_players: num_players,
                current_player: 0,
                next_marble_id: 1,
                current_marble: 0,
            }
        }

        pub fn add_marble(&mut self) -> Option<(usize, usize)> {
            // xxx clean up if possible
            if self.next_marble_id == 1 {
                self.left.push_back(self.current_marble);
                self.current_marble = self.next_marble_id;
                self.next_marble_id += 1;
                return None;
            }

            println!("add_marble");
            dbg!(&self.left);
            dbg!(self.current_marble);
            dbg!(&self.right);
            let mut ret = None;

            if self.next_marble_id % 23 == 0 {
                println!("score!");
                dbg!(self.current_player);
                dbg!(self.next_marble_id);

                dbg!(&self.left);
                dbg!(self.current_marble);
                dbg!(&self.right);

                for _ in 0..7 {
                    self.move_left();
                }

                dbg!(&self.left);
                dbg!(self.current_marble);
                dbg!(&self.right);

                ret = Some((
                    self.current_player,
                    self.next_marble_id + self.current_marble,
                ));

                if self.right.is_empty() {
                    self.wrap_around();
                }

                self.current_marble = self.right.pop_front().unwrap();
            } else {
                self.move_right();
                self.right.push_front(self.current_marble);

                self.current_marble = self.next_marble_id;

                // behavior i'm seeing:
                // before:
                // left [0] current 1 right []
                // after:
                // left [1] current 2 right [0 1]
                // what after _should_ look like instead:
                // left [0] current 2 right [1]
                // how is left becoming [1] ?

                // well, so we move right 1 and end up with
                // left [1] current 0 right []
                // wait i think we need a current_length thing
                // i mean i guess that's next_marble_id?
                // if left.len() == (self.next_marble_id - 1)
                //hmmm...

                // push current marble on the back of left
            }

            self.next_marble_id += 1;
            self.current_player = (self.current_player + 1) % self.num_players;

            ret
        }

        fn wrap_around(&mut self) {
            /*println!(
                "swapping left and right! before: {:?}, {:?}",
                &self.left, &self.right
            );
            println!("after: {:?}, {:?}", &self.left, &self.right);*/
            std::mem::swap(&mut self.left, &mut self.right);
        }

        fn move_right(&mut self) {
            println!(
                "moving right! before: {:?}, {}, {:?}",
                &self.left, self.current_marble, self.right,
            );
            if self.right.is_empty() {
                self.wrap_around();
            }

            self.left.push_back(self.current_marble);
            self.current_marble = self.right.pop_front().unwrap();

            println!(
                "after: {:?}, {}, {:?}",
                &self.left, self.current_marble, self.right,
            );
        }

        fn move_left(&mut self) {
            if self.left.is_empty() {
                self.wrap_around();
            }

            self.right.push_front(self.current_marble);
            self.current_marble = self.left.pop_back().unwrap();
        }
    }
}

fn marble_game_outcome_2(num_players: usize, last_marble: usize) -> usize {
    let mut marble_game = game::MarbleGame::new(num_players, last_marble);
    let mut scores = vec![0; num_players];

    for _ in 0..last_marble {
        if let Some((player_index, score)) = marble_game.add_marble() {
            scores[player_index] += score;
        }
    }

    *scores.iter().max().unwrap()
}

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

fn marble_game_outcome(num_players: usize, last_marble: u32) -> u32 {
    let mut current_player = 0;
    let mut current_marble_index: usize = 0;
    let mut scores = vec![0 as u32; num_players];

    let mut circle = Vec::with_capacity(last_marble as usize);
    circle.push(0);

    for i in 1..=last_marble {
        if i % 1000 == 0 {
            dbg!(i);
        }
        if i % 23 == 0 {
            scores[current_player] += i;

            let mut index_to_remove = (current_marble_index as i32 - 7) % circle.len() as i32;
            if index_to_remove < 0 {
                index_to_remove += circle.len() as i32;
            }

            scores[current_player] += circle.remove(index_to_remove as usize);
            current_marble_index = index_to_remove as usize;
        } else {
            current_marble_index = next_marble_position(current_marble_index, &circle);
            circle.insert(current_marble_index, i);
        }

        current_player = (current_player + 1) % num_players;
    }

    *scores.iter().max().unwrap()
}

pub fn nine_a() -> usize {
    //marble_game_outcome_2(413, 71082)
    //marble_game_outcome_2(10, 1618)
    //marble_game_outcome_2(9, 25)
    marble_game_outcome_2(9, 3)
}

pub fn nine_b() -> u32 {
    //marble_game_outcome(413, 7108200)
    5
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

    #[test]
    fn test_marble_game() {
        assert_eq!(marble_game_outcome(10, 1618), 8317);
        assert_eq!(marble_game_outcome(13, 7999), 146373);
        assert_eq!(marble_game_outcome(17, 1104), 2764);
        assert_eq!(marble_game_outcome(21, 6111), 54718);
        assert_eq!(marble_game_outcome(30, 5807), 37305);
    }
}
