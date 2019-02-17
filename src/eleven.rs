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

pub fn eleven_a() -> (usize, usize) {
    let mut grid = [[0; 300]; 300];

    let coordinates = (0..300)
        .flat_map(|x| (0..300).map(move |y| (x, y)))
        .collect::<Vec<(usize, usize)>>();

    for &(x, y) in &coordinates {
        grid[x][y] = power_level(x as u32 + 1, y as u32 + 1, SERIAL);
    }

    let mut square_powers = [[0; 300]; 300];

    for x in 0..298 {
        for y in 0..298 {
            let mut square_power = 0;
            for i in 0..3 {
                for j in 0..3 {
                    square_power += grid[x + i][y + j];
                }
            }

            square_powers[x][y] = square_power;
        }
    }

    let (x, y, _) = coordinates
        .iter()
        .map(|&(x, y)| (x, y, square_powers[x][y]))
        .max_by_key(|&(_, _, square_power)| square_power)
        .unwrap();

    (x + 1, y + 1)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(3, 5, 8), 4);
        assert_eq!(power_level(122, 79, 57), -5);
        assert_eq!(power_level(217, 196, 39), 0);
        assert_eq!(power_level(101, 153, 71), 4);
    }
}
