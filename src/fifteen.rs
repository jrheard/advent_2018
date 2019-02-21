use std::fs;

use crate::util;

// TODO implement partialeq by y, x
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
enum MonsterTeam {
    Goblin,
    Elf,
}

use MonsterTeam::*;

struct Monster {
    attack_power: u32,
    hp: u32,
    team: MonsterTeam,
    position: Position,
}

struct Game {
    open_spaces: Vec<Position>,
    monsters: Vec<Monster>,
    width: usize,
    height: usize,
}

impl Game {
    fn new() -> Game {
        let contents = fs::read_to_string("src/inputs/15_sample.txt").unwrap();

        let mut open_spaces = vec![];
        let mut monsters = vec![];

        for (y, line) in contents.lines().enumerate() {
            for (x, character) in line.trim().chars().enumerate() {
                match character {
                    '#' => continue,
                    '.' => open_spaces.push(Position { x, y }),
                    'G' | 'E' => {
                        open_spaces.push(Position { x, y });

                        monsters.push(Monster {
                            attack_power: 3,
                            hp: 200,
                            team: if character == 'G' { Goblin } else { Elf },
                            position: Position { x, y },
                        });
                    }
                    _ => panic!("unknown character {}!", character),
                };
            }
        }

        let height = contents.lines().count();
        let width = contents.lines().nth(0).unwrap().chars().count();

        Game {
            open_spaces,
            monsters,
            width,
            height,
        }
    }

    fn to_grid(&self) -> Vec<Vec<char>> {
        let mut grid = vec![vec!['#'; self.height]; self.width];

        for position in &self.open_spaces {
            grid[position.x][position.y] = '.';
        }

        for monster in &self.monsters {
            grid[monster.position.x][monster.position.y] =
                if monster.team == Goblin { 'G' } else { 'E' };
        }

        grid
    }
}

pub fn fifteen_a() {
    let game = Game::new();
    util::print_grid(&game.to_grid());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solutions() {}
}
