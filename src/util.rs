use hashbrown::HashMap;

pub fn frequencies<I, T>(x: I) -> HashMap<T, u32>
where
    I: Iterator<Item = T>,
    T: Copy + Eq + std::hash::Hash,
{
    let mut ret = HashMap::new();

    for item in x {
        let count = ret.entry(item).or_insert(0);
        *count += 1;
    }

    ret
}

pub fn most_common<I, T>(x: I) -> T
where
    I: Iterator<Item = T>,
    T: Copy + Eq + std::hash::Hash,
{
    let freqs = frequencies(x);
    *(freqs.iter().max_by_key(|(_, count)| *count).unwrap().0)
}

pub fn print_grid_with_bounds<T: std::fmt::Display>(
    grid: &[Vec<T>],
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
) {
    for y in min_y..=max_y {
        let mut row = String::new();

        for column in &grid[min_x..=max_x] {
            row.push_str(&format!("{}", column[y]));
        }

        println!("{}", row);
    }
}

#[allow(dead_code)]
pub fn print_grid<T: std::fmt::Display>(grid: &[Vec<T>]) {
    print_grid_with_bounds(grid, 0, grid.len() - 1, 0, grid[0].len() - 1);
}

#[cfg(test)]
mod test {
    use super::*;

    // via https://stackoverflow.com/questions/27582739/how-do-i-create-a-hashmap-literal
    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
        };
    );

    #[test]
    fn test_frequencies() {
        assert_eq!(
            frequencies("aabbccccd".chars()),
            map! { 'a' => 2, 'b' => 2, 'c' => 4, 'd' => 1}
        );

        assert_eq!(frequencies("abcabcaa".chars()), map! {'a' => 4, 'b' => 2, 'c' => 2});

        assert_eq!(frequencies("".chars()), HashMap::new());
    }
}
