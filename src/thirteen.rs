#![cfg_attr(feature = "cargo-clippy", allow(clippy::needless_range_loop))]
use std::fs;

#[derive(Clone, Copy, Debug)]
enum MineSpace {
    StraightVertical,   // |
    StraightHorizontal, // -
    CurveLeft,          // \
    CurveRight,         // /
    Intersection,       // +
    Empty,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

impl Direction {
    fn right(self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn left(self) -> Direction {
        match self {
            North => West,
            East => North,
            South => East,
            West => South,
        }
    }
}

#[derive(Debug)]
struct MineCart {
    x: usize,
    y: usize,
    turn_counter: u32,
    direction: Direction,
}

impl MineCart {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        MineCart {
            x,
            y,
            direction,
            turn_counter: 0,
        }
    }
}

fn parse_input(filename: &str) -> Mine {
    let contents = fs::read_to_string(filename).unwrap();

    let mut grid = vec![];
    let mut carts = vec![];

    for (y, line) in contents.lines().enumerate() {
        let mut row = vec![];
        for (x, character) in line.chars().enumerate() {
            let value = match character {
                ' ' => MineSpace::Empty,
                '>' => {
                    carts.push(MineCart::new(x, y, East));
                    MineSpace::StraightHorizontal
                }
                '<' => {
                    carts.push(MineCart::new(x, y, West));
                    MineSpace::StraightHorizontal
                }
                '^' => {
                    carts.push(MineCart::new(x, y, North));
                    MineSpace::StraightVertical
                }
                'v' => {
                    carts.push(MineCart::new(x, y, South));
                    MineSpace::StraightVertical
                }
                '|' => MineSpace::StraightVertical,
                '-' => MineSpace::StraightHorizontal,
                '\\' => MineSpace::CurveLeft,
                '/' => MineSpace::CurveRight,
                '+' => MineSpace::Intersection,
                _ => panic!("didn't recognize space {}!", character),
            };

            row.push(value);
        }
        grid.push(row);
    }

    // grid is indexed by [y][x], but I prefer indexing by [x][y], so let's rotate it.

    let mut ret_grid = vec![vec![MineSpace::Empty; grid.len()]; grid[0].len()];

    for x in 0..ret_grid.len() {
        for y in 0..ret_grid[0].len() {
            ret_grid[x][y] = grid[y][x];
        }
    }

    Mine { carts, grid: ret_grid }
}

struct Mine {
    carts: Vec<MineCart>,
    grid: Vec<Vec<MineSpace>>,
}

impl Mine {
    fn new(filename: &str) -> Mine {
        parse_input(filename)
    }

    /// Advances time one tick. Returns a vector containing the locations of any crashes that occurred.
    /// Removes any carts that were affected by a crash this tick from self.carts.
    fn tick(&mut self) -> Vec<(usize, usize)> {
        // "Carts all move at the same speed; they take turns moving a single step at a time.
        // They do this based on their current location: carts on the top row move first (acting
        // from left to right), then carts on the second row move (again from left to right),
        // then carts on the third row, and so on."
        self.carts.sort_by_key(|cart| (cart.y, cart.x));

        let mut cart_positions = self
            .carts
            .iter()
            .map(|cart| (cart.x, cart.y))
            .collect::<Vec<(usize, usize)>>();

        let mut crash_sites = vec![];

        for cart in &mut self.carts {
            // Remove our old position from `cart_positions`.
            cart_positions.retain(|&position| (cart.x, cart.y) != position);

            // There's a crash here, this cart is destroyed and will be removed after this loop!
            if crash_sites.contains(&(cart.x, cart.y)) {
                continue;
            }

            // Move the cart.
            match cart.direction {
                North => cart.y -= 1,
                East => cart.x += 1,
                South => cart.y += 1,
                West => cart.x -= 1,
            }

            // Check for crashes!
            for (x, y) in &cart_positions {
                if *x == cart.x && *y == cart.y {
                    crash_sites.push((*x, *y));
                }
            }

            // Adjust the cart's direction based on the piece of track it's now on.
            match self.grid[cart.x][cart.y] {
                MineSpace::CurveRight => {
                    cart.direction = match cart.direction {
                        North => East,
                        East => North,
                        South => West,
                        West => South,
                    }
                }
                MineSpace::CurveLeft => {
                    cart.direction = match cart.direction {
                        North => West,
                        East => South,
                        South => East,
                        West => North,
                    }
                }
                MineSpace::Intersection => {
                    // "Each time a cart has the option to turn (by arriving at any intersection),
                    // it turns left the first time, goes straight the second time, turns right the third time,
                    // and then repeats those directions starting again with left the fourth time,
                    // straight the fifth time, and so on."
                    if cart.turn_counter == 0 {
                        cart.direction = cart.direction.left();
                    } else if cart.turn_counter == 2 {
                        cart.direction = cart.direction.right();
                    }

                    cart.turn_counter += 1;
                    cart.turn_counter %= 3;
                }
                MineSpace::Empty => panic!("a cart fell off the map: {:?}", cart),
                _ => (),
            }

            cart_positions.push((cart.x, cart.y));
        }

        // Remove any carts that were involved in a crash this tick.
        self.carts.retain(|cart| !crash_sites.contains(&(cart.x, cart.y)));

        crash_sites
    }
}

/// After following their respective paths for a while, the carts eventually crash.
/// To help prevent crashes, you'd like to know the location of the first crash.
pub fn thirteen_a() -> (usize, usize) {
    let mut mine = Mine::new("src/inputs/13.txt");

    loop {
        let crashes = mine.tick();
        if !crashes.is_empty() {
            return crashes[0];
        }
    }
}

/// There isn't much you can do to prevent crashes in this ridiculous system.
/// However, by predicting the crashes, the Elves know where to be in advance and
/// instantly remove the two crashing carts the moment any crash occurs.
/// They can proceed like this for a while, but eventually, they're going to run out of carts.
/// It could be useful to figure out where the last cart that hasn't crashed will end up.
/// What is the location of the last cart at the end of the first tick where it is the only cart left?
pub fn thirteen_b() -> (usize, usize) {
    let mut mine = Mine::new("src/inputs/13.txt");

    loop {
        let _ = mine.tick();
        if mine.carts.len() == 1 {
            return (mine.carts[0].x, mine.carts[0].y);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(thirteen_a(), (113, 136));
        assert_eq!(thirteen_b(), (114, 136));
    }
}
