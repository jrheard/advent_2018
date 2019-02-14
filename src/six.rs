#![cfg_attr(feature = "cargo-clippy", allow(clippy::needless_range_loop))]

use std::fs;

use hashbrown::HashSet;
use serde_scan::scan;

use crate::util;

const SENTINEL_LOCATION_ID: i32 = -1;

/// Using only the Manhattan distance, determine the area around each coordinate
/// by counting the number of integer X,Y locations that are closest to that coordinate
/// (and aren't tied in distance to any other coordinate).

#[derive(Debug)]
struct Location {
    id: i32,
    x: usize,
    y: usize,
}

/// "The sum of the absolute values of the differences of the coordinates",
/// according to math stackexchange.
fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> u32 {
    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as u32
}

fn load_locations() -> Vec<Location> {
    let contents = fs::read_to_string("src/inputs/6.txt").unwrap();
    let mut id = 0;
    let mut locations = Vec::new();

    for line in contents.lines() {
        let (x, y) = scan!("{}, {}" <- line).unwrap();
        locations.push(Location { id, x, y });
        id += 1;
    }

    locations
}

struct LocationGrid {
    grid: Vec<Vec<i32>>,
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

/// Returns a LocationGrid whose values are all SENTINEL_LOCATION_ID.
fn initialize_grid(locations: &[Location]) -> LocationGrid {
    let mut xs = locations
        .iter()
        .map(|location| location.x)
        .collect::<Vec<_>>();
    let mut ys = locations
        .iter()
        .map(|location| location.y)
        .collect::<Vec<_>>();
    xs.sort();
    ys.sort();

    let (min_x, max_x) = (xs[0], *xs.last().unwrap());
    let (min_y, max_y) = (ys[0], *ys.last().unwrap());

    let grid = vec![vec![SENTINEL_LOCATION_ID; max_y as usize]; max_x as usize];

    LocationGrid {
        grid,
        min_x,
        min_y,
        max_x,
        max_y,
    }
}

/// What is the size of the largest area that isn't infinite?
pub fn six_a() -> u32 {
    let locations = load_locations();

    let mut location_grid = initialize_grid(&locations);

    let sentinel_location = Location {
        id: SENTINEL_LOCATION_ID,
        x: 0,
        y: 0,
    };

    // Calculate the ID of the closest location to each spot on the grid.

    for x in location_grid.min_x..location_grid.max_x {
        for y in location_grid.min_y..location_grid.max_y {
            let mut closest_location = &locations[0];
            let mut smallest_distance = std::u32::MAX;

            for location in &locations {
                let distance = manhattan_distance(location.x, location.y, x, y);

                if distance < smallest_distance {
                    smallest_distance = distance;
                    closest_location = location;
                } else if distance == smallest_distance {
                    closest_location = &sentinel_location;
                }
            }

            location_grid.grid[x as usize][y as usize] = closest_location.id;
        }
    }

    // If a Location's .id appears on the edge of the grid,
    // that means that it has the potential to claim an infinitely large area.

    let mut infinite_area_location_ids = HashSet::new();

    for &x in [location_grid.min_x, location_grid.max_x - 1].iter() {
        for y in location_grid.min_y..location_grid.max_y {
            infinite_area_location_ids.insert(location_grid.grid[x][y]);
        }
    }

    for &y in [location_grid.min_y, location_grid.max_y - 1].iter() {
        for x in location_grid.min_x..location_grid.max_x {
            infinite_area_location_ids.insert(location_grid.grid[x][y]);
        }
    }

    let candidate_spaces = location_grid
        .grid
        .iter()
        .flatten()
        .cloned()
        .filter(|&id| id != SENTINEL_LOCATION_ID && !infinite_area_location_ids.contains(&id));

    let freqs = util::frequencies(candidate_spaces);

    *(freqs.iter().max_by_key(|(_, &count)| count).unwrap().1)
}

/// On the other hand, if the coordinates are safe, maybe the best you can do
/// is try to find a region near as many coordinates as possible.
/// What is the size of the region containing all locations which have
/// a total distance to all given coordinates of less than 10000?
pub fn six_b() -> usize {
    let locations = load_locations();

    let mut location_grid = initialize_grid(&locations);

    // Mark each spot on the grid with the total distance to all Locations.
    for x in location_grid.min_x..location_grid.max_x {
        for y in location_grid.min_y..location_grid.max_y {
            location_grid.grid[x as usize][y as usize] = locations
                .iter()
                .map(|location| manhattan_distance(location.x, location.y, x, y) as i32)
                .sum();
        }
    }

    location_grid
        .grid
        .iter()
        .flatten()
        .filter(|&&total_distance| {
            total_distance != SENTINEL_LOCATION_ID && total_distance < 10_000
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solutions() {
        assert_eq!(six_a(), 4284);
        assert_eq!(six_b(), 35490);
    }

    #[test]
    fn test_manhattan_distance() {
        assert_eq!(manhattan_distance(5, 8, 10, 3), 10);
        assert_eq!(manhattan_distance(2, 4, 0, 6), 4);
    }

}
