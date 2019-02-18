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
    fn right(&self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn left(&self) -> Direction {
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

    let mut ret_grid = vec![vec![MineSpace::Empty; grid.len()]; grid[0].len()];

    for x in 0..ret_grid.len() {
        for y in 0..ret_grid[0].len() {
            ret_grid[x][y] = grid[y][x];
        }
    }

    Mine {
        carts,
        grid: ret_grid,
    }
}

struct Mine {
    carts: Vec<MineCart>,
    grid: Vec<Vec<MineSpace>>,
}

impl Mine {
    fn new(filename: &str) -> Mine {
        parse_input(filename)
    }

    /// Advances time one tick. Returns Ok(()) if there were no crashes, Err((x, y)) if there was a crash.
    fn tick(&mut self) -> Vec<(usize, usize)> {
        self.carts.sort_by_key(|cart| (cart.y, cart.x));

        let mut cart_positions = self
            .carts
            .iter()
            .map(|cart| (cart.x, cart.y, cart.direction))
            .collect::<Vec<(usize, usize, Direction)>>();

        let mut crash_sites = vec![];

        for cart in &mut self.carts {
            cart_positions.retain(|&position| (cart.x, cart.y, cart.direction) != position);

            if crash_sites.contains(&(cart.x, cart.y)) {
                continue;
            }

            match cart.direction {
                North => cart.y -= 1,
                East => cart.x += 1,
                South => cart.y += 1,
                West => cart.x -= 1,
            }

            // Check for crashes!
            for (x, y, direction) in &cart_positions {
                if *x == cart.x && *y == cart.y && *direction != cart.direction {
                    crash_sites.push((*x, *y));
                }
            }

            let space = self.grid[cart.x][cart.y];
            match space {
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

            cart_positions.push((cart.x, cart.y, cart.direction));
        }

        self.carts
            .retain(|cart| !crash_sites.contains(&(cart.x, cart.y)));

        crash_sites
    }
}

pub fn thirteen_b() -> (usize, usize) {
    let mut mine = Mine::new("src/inputs/13.txt");

    loop {
        let _ = mine.tick();
        if mine.carts.len() == 1 {
            return (mine.carts[0].x, mine.carts[0].y);
        }
    }
}

pub fn thirteen_a() -> (usize, usize) {
    let mut mine = Mine::new("src/inputs/13.txt");

    loop {
        let crashes = mine.tick();
        if !crashes.is_empty() {
            return crashes[0];
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
