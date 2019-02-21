use std::fs;

use hashbrown::HashSet;
use itertools::Itertools;

use crate::util;

// TODO implement partialeq by y, x
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    // TODO will we actually need this? no, right?
    fn distance_to(&self, other_position: &Position) -> usize {
        ((self.x as i32 - other_position.x as i32).abs()
            + (self.y as i32 - other_position.y as i32).abs()) as usize
    }

    fn adjacent_positions(
        &self,
        grid_width: usize,
        grid_height: usize,
        open_positions: &HashSet<Position>,
    ) -> Vec<Position> {
        let deltas = [(0, 1), (0, -1), (-1, 0), (1, 0)];

        deltas
            .iter()
            .map(|(delta_x, delta_y)| (self.x as i32 + delta_x, self.y as i32 + delta_y))
            .filter(|&(x, y)| {
                x >= 0
                    && x < grid_width as i32
                    && y >= 0
                    && y < grid_height as i32
                    && open_positions.contains(&Position {
                        x: x as usize,
                        y: y as usize,
                    })
            })
            .map(|(x, y)| Position {
                x: x as usize,
                y: y as usize,
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Clone)]
enum MonsterTeam {
    Goblin,
    Elf,
}

use MonsterTeam::*;

#[derive(Debug, Clone)]
struct Monster {
    attack_power: u32,
    hp: u32,
    team: MonsterTeam,
    position: Position,
}

#[derive(Debug)]
enum MonsterAction {
    MoveTo(Position),
    Attack(Monster),
    Blocked,
}

impl Monster {
    // XXXX implement a*?
    fn find_path_to(
        &self,
        destination: Position,
        open_positions: &HashSet<Position>,
    ) -> Vec<Vec<Position>> {
        vec![]
    }

    fn choose_action(
        &self,
        enemies: &Vec<Monster>,
        open_positions: &HashSet<Position>,
        grid_width: usize,
        grid_height: usize,
    ) -> MonsterAction {
        let mut targets_and_positions = vec![];

        for enemy in enemies {
            for position in
                enemy
                    .position
                    .adjacent_positions(grid_width, grid_height, open_positions)
            {
                targets_and_positions.push((enemy, position));
            }
        }

        if targets_and_positions.is_empty() {
            return MonsterAction::Blocked;
        }

        dbg!(self);
        //dbg!(targets_and_positions);

        // XXXX pathfind to each position
        // XXX filter out blocked destinations

        dbg!(targets_and_positions
            .iter()
            .min_by_key(|(_, position)| self.position.distance_to(&position))
            .unwrap());

        MonsterAction::Blocked
    }
}

struct Game {
    open_positions: HashSet<Position>,
    monsters: Vec<Monster>,
    width: usize,
    height: usize,
}

impl Game {
    fn new() -> Game {
        let contents = fs::read_to_string("src/inputs/15_sample.txt").unwrap();

        let mut open_positions = HashSet::new();
        let mut monsters = vec![];

        for (y, line) in contents.lines().enumerate() {
            for (x, character) in line.trim().chars().enumerate() {
                match character {
                    '#' => continue,
                    '.' => {
                        open_positions.insert(Position { x, y });
                    }
                    'G' | 'E' => {
                        // XXXXX put this in blocked_positionns instead
                        open_positions.insert(Position { x, y });

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
            open_positions,
            monsters,
            width,
            height,
        }
    }

    /// Returns a grid of chars, useful for printing the state of the game to the screen.
    fn to_grid(&self) -> Vec<Vec<char>> {
        let mut grid = vec![vec!['#'; self.height]; self.width];

        for position in &self.open_positions {
            grid[position.x][position.y] = '.';
        }

        for monster in &self.monsters {
            grid[monster.position.x][monster.position.y] =
                if monster.team == Goblin { 'G' } else { 'E' };
        }

        grid
    }

    fn tick(&mut self) {
        let mut new_monsters: Vec<Monster> = vec![];
        // XXXXX handle open_positions
        // XXXX do we have a separate occupied_spaces vec?

        for monster in self
            .monsters
            .iter()
            .sorted_by_key(|monster| (monster.position.y, monster.position.x))
        {
            let enemy_team = if monster.team == Elf { Goblin } else { Elf };
            let enemies = self
                .monsters
                .iter()
                .filter(|&monster| monster.team == enemy_team)
                .cloned()
                .collect::<Vec<Monster>>();

            if enemies.is_empty() {
                panic!("combat's over! TODO implement me");
                break;
            }

            let action =
                monster.choose_action(&enemies, &self.open_positions, self.width, self.height);
            dbg!(action);
        }
    }
}

pub fn fifteen_a() {
    let mut game = Game::new();
    util::print_grid(&game.to_grid());
    game.tick();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solutions() {}
}
