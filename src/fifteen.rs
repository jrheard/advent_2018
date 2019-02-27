// TODO consider adding narrative doc comments to this module and any other interesting ones
// explaining design iteration, perf improvements, etc

//use crate::util;

mod game {
    use std::collections::VecDeque;
    use std::fs;
    use std::iter::FromIterator;

    use hashbrown::HashMap;
    use hashbrown::HashSet;
    use itertools::Itertools;

    #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    struct Position {
        y: usize,
        x: usize,
    }

    impl Position {
        /// Returns an iterator over the Positions immediately north, south, east, and west of `self`.
        /// Only includes Positions that actually fit on the specified grid.
        fn neighbors(self, grid_width: usize, grid_height: usize) -> NeighborIterator {
            NeighborIterator {
                position: self,
                grid_width: grid_width,
                grid_height: grid_height,
                offset: 0,
            }
        }
    }

    // Got this idea from forrest.
    /// An iterator over the within-grid-bounds n/w/e/s Positions around `position`.
    struct NeighborIterator {
        position: Position,
        grid_width: usize,
        grid_height: usize,
        offset: usize,
    }

    impl Iterator for NeighborIterator {
        type Item = Position;

        fn next(&mut self) -> Option<Position> {
            // Deltas listed in reading order.
            let deltas = [(0, -1), (-1, 0), (1, 0), (0, 1)];
            let mut ret = None;

            while ret.is_none() && self.offset < 4 {
                let x = self.position.x as i32 + deltas[self.offset].0;
                let y = self.position.y as i32 + deltas[self.offset].1;
                if x >= 0 && x < self.grid_width as i32 && y >= 0 && y < self.grid_height as i32 {
                    ret = Some(Position {
                        x: x as usize,
                        y: y as usize,
                    });
                }
                self.offset += 1;
            }

            ret
        }
    }

    /// BFS as described in https://www.redblobgames.com/pathfinding/a-star/introduction.html .
    /// Returns a `came_from` map that can be used to calculate path costs.
    fn compute_came_from_map(
        origin: Position,
        destinations: &HashSet<Position>,
        open_positions: &HashSet<Position>,
        grid_width: usize,
        grid_height: usize,
    ) -> HashMap<Position, Position> {
        let mut frontier = VecDeque::new();
        frontier.push_back(origin);

        let mut came_from = HashMap::new();
        came_from.insert(origin, origin);

        let mut destinations_remaining = destinations.clone();

        while !frontier.is_empty() && !destinations_remaining.is_empty() {
            let current = frontier.pop_front().unwrap();

            for neighbor in current
                .neighbors(grid_width, grid_height)
                .filter(|position| open_positions.contains(position))
            {
                if !came_from.contains_key(&neighbor) {
                    frontier.push_back(neighbor);
                    came_from.insert(neighbor, current);
                    destinations_remaining.remove(&neighbor);
                }
            }
        }

        came_from
    }

    #[derive(Debug, PartialEq, Eq, Clone, Hash)]
    enum MonsterTeam {
        Goblin,
        Elf,
    }

    type MonsterId = usize;

    #[derive(Debug, Clone)]
    /// A monster! A goblin or an elf, which can wander around the game and attack other monsters.
    struct Monster {
        id: MonsterId,
        attack_power: u32,
        hp: i32,
        team: MonsterTeam,
        position: Position,
    }

    #[derive(Debug)]
    /// An enum expressing the possible actions that a monster can perform on its turn.
    enum MonsterAction {
        MoveTo(Position),
        Attack(MonsterId),
        MoveAndAttack(Position, MonsterId),
        Blocked,
    }

    impl Monster {
        /// Returns Some(target_monster_id) if it's possible to attack an enemy from `position`, None otherwise.
        fn calculate_attack_for_position(
            position: &Position,
            enemies: &[&Monster],
            grid_width: usize,
            grid_height: usize,
        ) -> Option<MonsterId> {
            let neighbors = position.neighbors(grid_width, grid_height);

            let mut enemy_neighbors = vec![];
            for neighbor in neighbors {
                for enemy in enemies {
                    if neighbor == enemy.position {
                        // We're next to an enemy!
                        enemy_neighbors.push(enemy);
                    }
                }
            }

            if enemy_neighbors.is_empty() {
                None
            } else {
                // "The adjacent target with the fewest hit points is selected; in a tie,
                // the adjacent target with the fewest hit points which is first in reading order is selected."
                enemy_neighbors.sort_by_key(|monster| (monster.hp, monster.position));
                Some(enemy_neighbors[0].id)
            }
        }

        /// Returns a MonsterAction indicating what the monster wants to do with its turn.
        fn choose_action(
            &self,
            enemies: &[&Monster],
            open_positions: &HashSet<Position>,
            grid_width: usize,
            grid_height: usize,
        ) -> MonsterAction {
            // Start by seeing if we're next to someone already.
            if let Some(enemy_id) =
                Monster::calculate_attack_for_position(&self.position, enemies, grid_width, grid_height)
            {
                return MonsterAction::Attack(enemy_id);
            }

            // Otherwise, try to find an open enemy-adjacent space to move to.
            let destinations = HashSet::from_iter(enemies.iter().flat_map(|enemy| {
                enemy
                    .position
                    .neighbors(grid_width, grid_height)
                    .filter(|position| open_positions.contains(position))
            }));

            if let Some(position) = self.choose_move(&destinations, &open_positions, grid_width, grid_height) {
                // We've found a path, and moving to `position` will get us closer to our destination...
                if let Some(enemy_id) =
                    Monster::calculate_attack_for_position(&position, enemies, grid_width, grid_height)
                {
                    // ...and we can attack someone once we get there!
                    MonsterAction::MoveAndAttack(position, enemy_id)
                } else {
                    // ...but there aren't any enemies adjacent to that position, so we should just move there and that's our turn.
                    MonsterAction::MoveTo(position)
                }
            } else {
                // Either there was nowhere to go or no path was found to any destination.
                MonsterAction::Blocked
            }
        }

        /// Returns Some(position) representing the neighboring position that this monster should move to
        /// in order to pursue an enemy. Returns None if there are no unblocked paths to the monster's enemies.
        fn choose_move(
            &self,
            destinations: &HashSet<Position>,
            open_positions: &HashSet<Position>,
            grid_width: usize,
            grid_height: usize,
        ) -> Option<Position> {
            if destinations.is_empty() {
                return None;
            }

            let neighbors = self
                .position
                .neighbors(grid_width, grid_height)
                .filter(|position| open_positions.contains(position));

            let mut smallest_cost = std::usize::MAX;
            let mut chosen_move = None;
            let mut chosen_destination = self.position;

            for neighbor in neighbors {
                let came_from = compute_came_from_map(neighbor, destinations, open_positions, grid_width, grid_height);

                for &destination in destinations {
                    if !came_from.contains_key(&destination) {
                        // This destination's unreachable from `neighbor`, skip it!
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
                        chosen_destination = destination;
                    } else if path_cost == smallest_cost {
                        match chosen_move {
                        Some(position)
                            // "If multiple steps would put the unit equally closer to its destination,
                            // the unit chooses the step which is first in reading order."
                            if neighbor < position ||
                            // "If multiple squares are in range and tied for being reachable
                            // in the fewest steps, the square which is first in reading order is chosen."
                            destination < chosen_destination =>
                        {
                            chosen_move = Some(neighbor);
                            chosen_destination = destination;
                        }
                        _ => (),
                    };
                    }
                }
            }

            chosen_move
        }
    }

    #[derive(Debug)]
    pub struct Game {
        // Positions that aren't blocked off by cave walls.
        open_positions: HashSet<Position>,
        // Positions that are open _and_ don't have a monster on them at the moment.
        // A subset of `open_positions`.
        unoccupied_positions: HashSet<Position>,
        monsters: HashMap<usize, Monster>,
        width: usize,
        height: usize,
    }

    //use std::{thread, time};

    impl Game {
        /// Performs a round of combat as specified in the day 15 writeup.
        /// Returns true if the game's over, false otherwise.
        pub fn tick(&mut self) -> bool {
            // "The order in which units take their turns within a round is the reading order
            // of their starting positions in that round, regardless of the type of unit
            // or whether other units have moved after the round started."
            let sorted_monster_ids = self
                .monsters
                .iter()
                .sorted_by_key(|(_, monster)| monster.position)
                .map(|(id, _)| id)
                .cloned()
                .collect::<Vec<usize>>();

            for id in sorted_monster_ids {
                let monster = match self.monsters.get(&id) {
                    Some(monster) => monster,
                    None => {
                        // The monster was killed earlier this round.
                        continue;
                    }
                };

                let other_monsters = self
                    .monsters
                    .values()
                    .filter(|other_monster| other_monster.id != monster.id)
                    .collect::<Vec<&Monster>>();

                let enemy_team = if monster.team == MonsterTeam::Elf {
                    MonsterTeam::Goblin
                } else {
                    MonsterTeam::Elf
                };

                // "Each unit begins its turn by identifying all possible targets (enemy units)."
                let enemies = other_monsters
                    .iter()
                    .filter(|&monster| monster.team == enemy_team)
                    .cloned()
                    .collect::<Vec<&Monster>>();

                if enemies.is_empty() {
                    // "If no targets remain, combat ends."
                    return true;
                }

                let action = monster.choose_action(&enemies, &self.unoccupied_positions, self.width, self.height);

                let attack_power = monster.attack_power;
                let old_position = monster.position;

                // Moves `monster` to `new_position`.
                let perform_move = |self_: &mut Game, new_position| {
                    self_
                        .monsters
                        .entry(id)
                        .and_modify(|monster| monster.position = new_position);
                    self_.unoccupied_positions.remove(&new_position);
                    self_.unoccupied_positions.insert(old_position);
                };

                // Attacks the Monster identified by `target_id` and removes it from the game if it dies.
                let perform_attack = |self_: &mut Game, target_id| {
                    self_
                        .monsters
                        .entry(target_id)
                        .and_modify(|enemy| enemy.hp -= attack_power as i32);

                    if self_.monsters[&target_id].hp <= 0 {
                        // It's dead!
                        self_.unoccupied_positions.insert(self_.monsters[&target_id].position);
                        self_.monsters.remove(&target_id);
                    }
                };

                match action {
                    MonsterAction::MoveTo(position) => {
                        perform_move(self, position);
                    }
                    MonsterAction::Attack(target_id) => {
                        perform_attack(self, target_id);
                    }
                    MonsterAction::MoveAndAttack(position, target_id) => {
                        perform_move(self, position);
                        perform_attack(self, target_id);
                    }
                    MonsterAction::Blocked => (),
                }
            }

            //util::print_grid(&self.to_grid());
            //thread::sleep(time::Duration::from_millis(50));

            false
        }

        /// Parses the puzzle input file into a Game struct.
        pub fn new(filename: &str, elf_attack_power: u32) -> Game {
            let contents = fs::read_to_string(filename).unwrap();

            let mut next_id = 0;
            let mut open_positions = HashSet::new();
            let mut unoccupied_positions = HashSet::new();
            let mut monsters = HashMap::new();

            for (y, line) in contents.lines().enumerate() {
                for (x, character) in line.trim().chars().enumerate() {
                    match character {
                        '#' => continue,
                        // An open space!
                        '.' => {
                            open_positions.insert(Position { x, y });
                            unoccupied_positions.insert(Position { x, y });
                        }
                        // A goblin or elf!
                        'G' | 'E' => {
                            open_positions.insert(Position { x, y });

                            let attack_power = if character == 'G' { 3 } else { elf_attack_power };

                            monsters.insert(
                                next_id,
                                Monster {
                                    id: next_id,
                                    attack_power: attack_power,
                                    hp: 200,
                                    team: if character == 'G' {
                                        MonsterTeam::Goblin
                                    } else {
                                        MonsterTeam::Elf
                                    },
                                    position: Position { x, y },
                                },
                            );

                            next_id += 1;
                        }
                        _ => panic!("unknown character {}!", character),
                    };
                }
            }

            let height = contents.lines().count();
            let width = contents.lines().nth(0).unwrap().chars().count();

            Game {
                open_positions,
                unoccupied_positions,
                monsters,
                width,
                height,
            }
        }

        pub fn num_elves(&self) -> usize {
            self.monsters
                .values()
                .filter(|monster| monster.team == MonsterTeam::Elf)
                .count()
        }

        pub fn summed_health(&self) -> usize {
            self.monsters.values().map(|monster| monster.hp).sum::<i32>() as usize
        }

        /// Returns a grid of chars, useful for printing the state of the game to the screen.
        #[allow(dead_code)]
        pub fn to_grid(&self) -> Vec<Vec<char>> {
            let mut grid = vec![vec!['#'; self.height]; self.width];

            for position in &self.open_positions {
                grid[position.x][position.y] = '.';
            }

            for monster in self.monsters.values() {
                grid[monster.position.x][monster.position.y] =
                    if monster.team == MonsterTeam::Goblin { 'G' } else { 'E' };
            }

            grid
        }
    }
}

use game::Game;

/// You need to determine the outcome of the battle: the number of full rounds that were completed
/// (not counting the round in which combat ends) multiplied by the sum of the hit points of all
/// remaining units at the moment combat ends. (Combat only ends when a unit finds no targets during its turn.)
/// What is the outcome of the combat described in your puzzle input?
pub fn fifteen_a(filename: &str) -> usize {
    let mut game = Game::new(filename, 3);

    let mut i = 0;
    loop {
        if game.tick() {
            return i * game.summed_health();
        }

        i += 1;
    }
}

/// After increasing the Elves' attack power until it is just barely enough for them to win
/// without any Elves dying, what is the outcome of the combat described in your puzzle input?
pub fn fifteen_b(filename: &str) -> usize {
    let mut attack_power = 3;

    loop {
        let mut game = Game::new(filename, attack_power);
        let num_alive_elves_before_combat = game.num_elves();

        let mut i = 0;
        loop {
            let game_over = game.tick();

            let num_alive_elves = game.num_elves();

            if num_alive_elves < num_alive_elves_before_combat {
                // Oh no, an elf died! Buff the elves by 1 attack power and try again.
                attack_power += 1;
                break;
            }

            if game_over {
                // Combat ended and all the elves survived! Compute our combat outcome and return it!
                return i * game.summed_health();
            }

            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(fifteen_a("src/inputs/15_sample_2.txt"), 27730);
        assert_eq!(fifteen_a("src/inputs/15_sample_3.txt"), 36334);
        assert_eq!(fifteen_a("src/inputs/15_sample_4.txt"), 39514);
        assert_eq!(fifteen_a("src/inputs/15_sample_5.txt"), 28944);
        assert_eq!(fifteen_a("src/inputs/15_sample_6.txt"), 18740);
        assert_eq!(fifteen_a("src/inputs/15_sample_9.txt"), 27755);
        assert_eq!(fifteen_a("src/inputs/15.txt"), 229798);
        assert_eq!(fifteen_b("src/inputs/15.txt"), 52972);
    }
}
