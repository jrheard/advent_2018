use std::fs;

#[derive(Clone, Copy)]
enum MineSpace {
    StraightVertical,   // |
    StraightHorizontal, // -
    CurveLeft,          // \
    CurveRight,         // /
    Intersection,       // +
    Empty,
}

enum Direction {
    North,
    East,
    South,
    West,
}

struct MineCart {
    x: usize,
    y: usize,
    direction: Direction,
}

type Grid = Vec<Vec<MineSpace>>;

fn parse_input() -> (Grid, Vec<MineCart>) {
    let contents = fs::read_to_string("src/inputs/13_sample.txt").unwrap();

    let mut grid = vec![];
    let mut carts = vec![];

    for (y, line) in contents.lines().enumerate() {
        let mut row = vec![];
        for (x, character) in line.chars().enumerate() {
            let value = match character {
                ' ' => MineSpace::Empty,
                '>' => {
                    carts.push(MineCart {
                        x,
                        y,
                        direction: Direction::East,
                    });
                    MineSpace::StraightHorizontal
                }
                '<' => {
                    carts.push(MineCart {
                        x,
                        y,
                        direction: Direction::West,
                    });
                    MineSpace::StraightHorizontal
                }
                '^' => {
                    carts.push(MineCart {
                        x,
                        y,
                        direction: Direction::North,
                    });
                    MineSpace::StraightVertical
                }
                'v' => {
                    carts.push(MineCart {
                        x,
                        y,
                        direction: Direction::South,
                    });
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

    (ret_grid, carts)
}

pub fn thirteen_a() -> i32 {
    5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solutions() {}
}
