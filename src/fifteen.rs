use hashbrown::HashMap;
use std::collections::VecDeque;
use std::fs;

use hashbrown::HashSet;
use itertools::Itertools;

use crate::util;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd)]
struct Position {
    y: usize,
    x: usize,
}

impl Position {
    fn neighbors(
        &self,
        open_positions: &HashSet<Position>,
        grid_width: usize,
        grid_height: usize,
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
    fn choose_move(
        &self,
        destinations: &HashSet<Position>,
        open_positions: &HashSet<Position>,
        grid_width: usize,
        grid_height: usize,
    ) -> Option<Position> {
        /// BFS as described in https://www.redblobgames.com/pathfinding/a-star/introduction.html .
        /// Returns a `came_from` map that can be used to calculate path costs.
        fn compute_came_from_map(
            origin: Position,
            open_positions: &HashSet<Position>,
            grid_width: usize,
            grid_height: usize,
        ) -> HashMap<Position, Position> {
            let mut frontier = VecDeque::new();
            frontier.push_back(origin);

            let mut came_from = HashMap::new();
            came_from.insert(origin, origin);

            while !frontier.is_empty() {
                let current = frontier.pop_front().unwrap();

                // TODO early exit once we've seen all of the destinations?
                for neighbor in current.neighbors(open_positions, grid_width, grid_height) {
                    // TODO do we need do anything special if came_from contains neighbor and came_from[neighbor] is > current?
                    if !came_from.contains_key(&neighbor) {
                        frontier.push_back(neighbor);
                        came_from.insert(neighbor, current);
                    }
                }
            }

            came_from
        }

        let neighbors = self
            .position
            .neighbors(open_positions, grid_width, grid_height);

        let mut smallest_cost = std::usize::MAX;
        let mut chosen_move = None;

        for &neighbor in &neighbors {
            let came_from =
                compute_came_from_map(neighbor, open_positions, grid_width, grid_height);

            for &destination in destinations {
                if !came_from.contains_key(&destination) {
                    // This destination's unreachable, skip it!
                    continue;
                }

                // Walk the path from `destination` to `neighbor` and count its length.
                let mut path_cost = 0;
                let mut current_position = destination;

                while current_position != neighbor {
                    path_cost += 1;
                    current_position = came_from[&current_position];
                }

                // If this is the shortest path we've seen so far, record it.
                if path_cost < smallest_cost {
                    smallest_cost = path_cost;
                    chosen_move = Some(neighbor);
                }
                // "If multiple steps would put the unit equally closer to its destination,
                // the unit chooses the step which is first in reading order."
                else if path_cost == smallest_cost {
                    match chosen_move {
                        Some(position) if neighbor < position => {
                            chosen_move = Some(neighbor);
                        }
                        _ => (),
                    };
                }
            }
        }

        chosen_move
    }

    fn choose_action(
        &self,
        enemies: &Vec<Monster>,
        open_positions: &HashSet<Position>,
        grid_width: usize,
        grid_height: usize,
    ) -> MonsterAction {
        let mut destinations = HashSet::new();

        // todo iterator-ize
        for enemy in enemies {
            for position in enemy
                .position
                .neighbors(open_positions, grid_width, grid_height)
            {
                destinations.insert(position);
            }
        }

        if destinations.is_empty() {
            return MonsterAction::Blocked;
        }

        match self.choose_move(&destinations, &open_positions, grid_width, grid_height) {
            Some(position) => MonsterAction::MoveTo(position),
            None => MonsterAction::Blocked,
        }
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
        // XXXXX handle open_positions
        // XXXX do we have a separate occupied_spaces vec?
        self.monsters
            .sort_by_key(|monster| (monster.position.y, monster.position.x));

        // TODO while loop?
        for i in 0..self.monsters.len() {
            let (left, right) = self.monsters.split_at_mut(i);
            let (monster, right) = right.split_first_mut().unwrap();
            let other_monsters = left.iter().chain(right.iter());

            let enemy_team = if monster.team == Elf { Goblin } else { Elf };
            let enemies = other_monsters
                .filter(|&monster| monster.team == enemy_team)
                .cloned()
                .collect::<Vec<Monster>>();

            if enemies.is_empty() {
                panic!("combat's over! TODO implement me");
                break;
            }

            //dbg!(&monster);
            let action =
                monster.choose_action(&enemies, &self.open_positions, self.width, self.height);
            //dbg!(&action);

            match action {
                MonsterAction::MoveTo(position) => {
                    monster.position = position;
                }
                MonsterAction::Attack(monster) => panic!("eep"),
                MonsterAction::Blocked => (),
            }
        }

        // TODO remove dead monsters
        // TODO ignore dead monsters when pathfinding
    }
}

pub fn fifteen_a() {
    let mut game = Game::new();
    util::print_grid(&game.to_grid());

    for _ in 0..3 {
        game.tick();
        util::print_grid(&game.to_grid());
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solutions() {}
}
