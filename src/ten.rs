use std::fs;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Point {
    fn new(input_line: &str) -> Self {
        lazy_static! {
            //#static ref pattern: Regex = Regex::new(r"(?x)
                //.*\<[ ]?(?P<x>-?[0-9]+),  ?(?P<y>-?[0-9]+)> velocity=< ?(?P<dx>-?[0-9]+),  ?(?P<dy>-?[0-9]+)>$").unwrap();
            static ref pattern: Regex = Regex::new(r".*< ?(?P<x>-?[0-9]+),  ?(?P<y>-?[0-9]+)>.*< ?(?P<dx>-?[0-9]+),  ?(?P<dy>-?[0-9]+)>").unwrap();
        }

        let caps = pattern.captures(input_line).unwrap();
        let value = |match_name| {
            caps.name(match_name)
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap()
        };

        Point {
            x: value("x"),
            y: value("y"),
            dx: value("dx"),
            dy: value("dy"),
        }
    }
}

struct Grid {
    points: Vec<Point>,
    // TODO update these each turn
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Grid {
    fn new(points: Vec<Point>) -> Self {
        let min_x = points.iter().map(|point| point.x).min().unwrap();
        let max_x = points.iter().map(|point| point.x).max().unwrap();
        let min_y = points.iter().map(|point| point.y).min().unwrap();
        let max_y = points.iter().map(|point| point.y).max().unwrap();

        Grid {
            min_x,
            max_x,
            min_y,
            max_y,
            points,
        }
    }

    fn print(&self) {
        let mut grid = vec![
            vec![false; (self.max_y - self.min_y) as usize + 1];
            (self.max_x - self.min_x) as usize + 1
        ];

        for point in &self.points {
            grid[(point.x - self.min_x) as usize][(point.y - self.min_y) as usize] = true;
        }

        for y in 0..grid[0].len() {
            let mut row = String::new();

            for x in 0..grid.len() {
                row.push(if grid[x][y] { 'X' } else { '.' });
            }

            println!("{}", row);
        }
    }
}

pub fn ten_a() -> u32 {
    let contents = fs::read_to_string("src/inputs/10_sample.txt").unwrap();
    let points: Vec<Point> = contents.lines().map(Point::new).collect();

    let mut grid = Grid::new(points);
    grid.print();

    5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(ten_a(), 5);
    }

    #[test]
    fn test_point_new() {
        assert_eq!(
            Point::new("position=< 7, -2> velocity=<-1,  1>"),
            Point {
                x: 7,
                y: -2,
                dx: -1,
                dy: 1
            }
        );

        assert_eq!(
            Point::new("position=<-6, 10> velocity=< 2, -2>"),
            Point {
                x: -6,
                y: 10,
                dx: 2,
                dy: -2
            }
        )
    }
}
