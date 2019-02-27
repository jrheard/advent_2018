mod game {
    use std::collections::VecDeque;

    /// An implementation of day 9's weird circle-of-marbles game.
    /// Uses two deques, `left` and `right`, to represent the circle.
    /// The marble on the back of `left` is the "current marble" in the game's terminology.
    /// The game slides through the circle, moving values from self.right to self.left as it moves right,
    /// and from self.left to self.right as it moves left.
    /// If it reaches the end of `self.left` or `self.right`, it swaps them.
    /// It's a circle!
    pub struct MarbleGame {
        left: VecDeque<usize>,
        right: VecDeque<usize>,

        num_players: usize,
        current_player: usize,

        next_marble_id: usize,
    }

    impl MarbleGame {
        pub fn new(num_players: usize, last_marble: usize) -> MarbleGame {
            let mut game = MarbleGame {
                left: VecDeque::with_capacity(last_marble),
                right: VecDeque::with_capacity(last_marble),
                num_players: num_players,
                current_player: 0,
                next_marble_id: 1,
            };
            game.left.push_back(0);

            game
        }

        /// Adds a marble to the circle.
        ///
        /// Returns Some((player_id, points)) if a player scored this round.
        /// Returns None if nobody scored this round.
        pub fn add_marble(&mut self) -> Option<(usize, usize)> {
            let mut ret = None;

            if self.next_marble_id % 23 == 0 {
                // The current player scored some points!
                for _ in 0..7 {
                    self.move_left();
                }

                if self.left.is_empty() {
                    self.wrap_around();
                }

                ret = Some((self.current_player, self.next_marble_id + self.left.pop_back().unwrap()));

                // "The marble located immediately clockwise of the marble that was removed becomes the new current marble."
                self.move_right();
            } else {
                // This isn't a score-getting turn, so just place a marble normally.
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
    marble_game_outcome(413, 71082)
}

pub fn nine_b() -> usize {
    marble_game_outcome(413, 7108200)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(nine_a(), 416424);
        assert_eq!(nine_b(), 3498287922);
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
