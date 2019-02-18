fn power_level(x: u32, y: u32, serial: u32) -> i32 {
    // Find the fuel cell's rack ID, which is its X coordinate plus 10.
    let rack_id = x + 10;

    // Begin with a power level of the rack ID times the Y coordinate.
    let mut power = rack_id * y;

    // Increase the power level by the value of the grid serial number (your puzzle input).
    power += serial;

    // Set the power level to itself multiplied by the rack ID.
    power *= rack_id;

    // Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    power %= 1000;
    power /= 100;

    // Subtract 5 from the power level.
    power as i32 - 5
}

const SERIAL: u32 = 6303;
const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = 300;

fn coordinates() -> Vec<(usize, usize)> {
    (0..GRID_WIDTH)
        .flat_map(|x| (0..GRID_HEIGHT).map(move |y| (x, y)))
        .collect::<Vec<(usize, usize)>>()
}

fn make_grid() -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0; GRID_HEIGHT]; GRID_WIDTH];

    for (x, y) in coordinates() {
        grid[x][y] = power_level(x as u32 + 1, y as u32 + 1, SERIAL);
    }

    grid
}

fn square_powers(grid: &Vec<Vec<i32>>, square_side_len: usize) -> Vec<Vec<i32>> {
    let mut summed_grid = vec![vec![0; GRID_HEIGHT]; GRID_WIDTH];

    for y in 0..=(GRID_HEIGHT - square_side_len) {
        // Start by filling in the far left square.

        let mut square_power = 0;

        if y == 0 {
            // Do it from scratch, because there's no previously-filled-in row above us.
            for i in 0..square_side_len {
                for j in 0..square_side_len {
                    square_power += grid[i][y + j];
                }
            }
        } else {
            // Cheat by peeking at the row above us.
            square_power = summed_grid[0][y - 1];

            for x in 0..square_side_len {
                square_power -= grid[x][y - 1];
                square_power += grid[x][y + square_side_len - 1];
            }
        }

        summed_grid[0][y] = square_power;

        // Then fill in the rest of the row.
        for x in 1..=(GRID_WIDTH - square_side_len) {
            let mut square_power = summed_grid[x - 1][y];

            for i in 0..square_side_len {
                square_power -= grid[x - 1][y + i];
                square_power += grid[x + square_side_len - 1][y + i];
            }

            summed_grid[x][y] = square_power;
        }
    }

    summed_grid
}

pub fn eleven_a() -> (usize, usize) {
    let grid = make_grid();
    let summed_grid = square_powers(&grid, 3);

    let (x, y, _) = coordinates()
        .iter()
        .map(|&(x, y)| (x, y, summed_grid[x][y]))
        .max_by_key(|&(_, _, square_power)| square_power)
        .unwrap();

    (x + 1, y + 1)
}

pub fn eleven_b() -> (usize, usize, usize) {
    let grid = make_grid();
    let mut max_power = 0;
    let mut x = 0;
    let mut y = 0;
    let mut square_side_len = 0;

    for size in 1..=300 {
        let summed_grid = square_powers(&grid, size);

        let (xx, yy, square_power) = coordinates()
            .iter()
            .map(|&(x, y)| (x, y, summed_grid[x][y]))
            .max_by_key(|&(_, _, square_power)| square_power)
            .unwrap();

        if square_power > max_power {
            x = xx;
            y = yy;
            max_power = square_power;
            square_side_len = size;
        }
    }

    (x + 1, y + 1, square_side_len)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(eleven_a(), (243, 27));
        assert_eq!(eleven_b(), (284, 172, 12));
    }

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
