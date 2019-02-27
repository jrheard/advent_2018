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

/// See https://en.wikipedia.org/wiki/Summed-area_table
fn make_summed_area_table(grid: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let mut table = vec![vec![0; GRID_HEIGHT]; GRID_WIDTH];

    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            let above = if y > 0 { table[x][y - 1] } else { 0 };

            let left = if x > 0 { table[x - 1][y] } else { 0 };

            let above_left = if x > 0 && y > 0 { table[x - 1][y - 1] } else { 0 };

            table[x][y] = grid[x][y] + above + left - above_left;
        }
    }

    table
}

fn square_with_most_power(table: &[Vec<i32>], square_side_len: usize) -> (usize, usize, i32) {
    let mut ret_x = 0;
    let mut ret_y = 0;
    let mut most_power = std::i32::MIN;

    // Use the summed-area-table algorithm to calculate the total power
    // for each square.

    for x in square_side_len - 1..GRID_WIDTH {
        for y in square_side_len - 1..GRID_HEIGHT {
            // The square's power is:
            // * The value of the summed-area-table's entry for the bottom-right coordinate of the square
            // * minus the value of the summed-area-table's entry for the coordinate `square_side_len` due north
            // * minus the value of the summed-area-table's entry for the coordinate `square_side_len` due west
            // * plus the value of the summed-area-table's entry for the coordinate `square_side_len` due northwest

            let above = if y >= square_side_len {
                table[x][y - square_side_len]
            } else {
                0
            };

            let left = if x >= square_side_len {
                table[x - square_side_len][y]
            } else {
                0
            };

            let above_left = if x >= square_side_len && y >= square_side_len {
                table[x - square_side_len][y - square_side_len]
            } else {
                0
            };

            let square_power = table[x][y] - above - left + above_left;

            if square_power > most_power {
                most_power = square_power;
                ret_x = x;
                ret_y = y;
            }
        }
    }

    (ret_x - (square_side_len - 1), ret_y - (square_side_len - 1), most_power)
}

/// Each fuel cell has a coordinate ranging from 1 to 300 in both the X (horizontal)
/// and Y (vertical) direction. In X,Y notation, the top-left cell is 1,1,
/// and the top-right cell is 300,1.

/// Your goal is to find the 3x3 square which has the largest total power.
/// What is the X,Y coordinate of the top-left fuel cell of the 3x3 square with the largest total power?
pub fn eleven_a() -> (usize, usize) {
    let grid = make_grid();
    let table = make_summed_area_table(&grid);
    let (x, y, _) = square_with_most_power(&table, 3);

    (x + 1, y + 1)
}

/// You now must find the square of any size with the largest total power. Identify this
/// square by including its size as a third parameter after the top-left coordinate:
/// a 9x9 square with a top-left corner of 3,5 is identified as 3,5,9.
/// What is the X,Y,size identifier of the square with the largest total power?
pub fn eleven_b() -> (usize, usize, usize) {
    let grid = make_grid();
    let table = make_summed_area_table(&grid);
    let mut max_power = 0;
    let mut x = 0;
    let mut y = 0;
    let mut square_side_len = 0;

    for size in 1..=300 {
        let (xx, yy, square_power) = square_with_most_power(&table, size);

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
