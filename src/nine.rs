mod game {
    use std::collections::VecDeque;

    pub struct MarbleGame {
        left: VecDeque<usize>,
        right: VecDeque<usize>,

        num_players: usize,
        current_player: usize,

        next_marble_id: usize,
    }

    impl MarbleGame {
        pub fn new(num_players: usize, last_marble: usize) -> MarbleGame {
            MarbleGame {
                left: VecDeque::with_capacity(last_marble),
                right: VecDeque::with_capacity(last_marble),
                num_players: num_players,
                current_player: 0,
                next_marble_id: 0,
            }
        }

        pub fn add_marble(&mut self) -> Option<(usize, usize)> {
            // xxx clean up if possible
            if self.next_marble_id <= 1 {
                self.left.push_back(self.next_marble_id);
                self.next_marble_id += 1;
                return None;
            }

            let mut ret = None;

            if self.next_marble_id % 23 == 0 {
                for _ in 0..7 {
                    self.move_left();
                }

                if self.left.is_empty() {
                    self.wrap_around();
                }

                ret = Some((
                    self.current_player,
                    self.next_marble_id + self.left.pop_back().unwrap(),
                ));

                self.move_right();
            } else {
                self.move_right();
                self.left.push_back(self.next_marble_id);
            }

            self.next_marble_id += 1;
            self.current_player = (self.current_player + 1) % self.num_players;

            ret
        }

        fn wrap_around(&mut self) {
            std::mem::swap(&mut self.left, &mut self.right);
        }

        fn move_right(&mut self) {
            if self.right.is_empty() {
                self.wrap_around();
            }

            self.left.push_back(self.right.pop_front().unwrap());
        }

        fn move_left(&mut self) {
            if self.left.is_empty() {
                self.wrap_around();
            }

            self.right.push_front(self.left.pop_back().unwrap());
        }
    }
}

fn marble_game_outcome(num_players: usize, last_marble: usize) -> usize {
    let mut marble_game = game::MarbleGame::new(num_players, last_marble);
    let mut scores = vec![0; num_players];

    for _ in 0..last_marble {
        if let Some((player_index, score)) = marble_game.add_marble() {
            scores[player_index] += score;
        }
    }

    *scores.iter().max().unwrap()
}

pub fn nine_a() -> usize {
    //marble_game_outcome(413, 71082)
    // marble_game_outcome(10, 1618)
    marble_game_outcome(17, 1104)
    //marble_game_outcome(9, 25)
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
    fn test_marble_game() {
        assert_eq!(marble_game_outcome(10, 1618), 8317);
        assert_eq!(marble_game_outcome(13, 7999), 146373);
        assert_eq!(marble_game_outcome(17, 1104), 2764);
        assert_eq!(marble_game_outcome(21, 6111), 54718);
        assert_eq!(marble_game_outcome(30, 5807), 37305);
    }
}
