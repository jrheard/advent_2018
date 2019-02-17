// TODO cleanup
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
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Grid {
    fn bounds(points: &[Point]) -> (i32, i32, i32, i32) {
        (
            points.iter().map(|point| point.x).min().unwrap(),
            points.iter().map(|point| point.x).max().unwrap(),
            points.iter().map(|point| point.y).min().unwrap(),
            points.iter().map(|point| point.y).max().unwrap(),
        )
    }

    fn new(points: Vec<Point>) -> Self {
        let (min_x, max_x, min_y, max_y) = Grid::bounds(&points);

        Grid {
            min_x,
            max_x,
            min_y,
            max_y,
            points,
        }
    }

    fn advance(&mut self) {
        for point in &mut self.points {
            point.x += point.dx;
            point.y += point.dy;
        }

        let (min_x, max_x, min_y, max_y) = Grid::bounds(&self.points);
        self.min_x = min_x;
        self.max_x = max_x;
        self.min_y = min_y;
        self.max_y = max_y;
    }

    fn to_vec(&self) -> Vec<Vec<bool>> {
        let mut grid = vec![
            vec![false; (self.max_y - self.min_y) as usize + 1];
            (self.max_x - self.min_x) as usize + 1
        ];

        for point in &self.points {
            grid[(point.x - self.min_x) as usize][(point.y - self.min_y) as usize] = true;
        }

        grid
    }

    fn print(&self) {
        let grid = self.to_vec();

        for y in 0..grid[0].len() {
            let mut row = String::new();

            for column in grid.iter() {
                row.push(if column[y] { 'X' } else { '.' });
            }

            println!("{}", row);
        }
    }
}

const LONG_LINE_THRESHOLD: u32 = 5;

fn num_lines(grid: Vec<Vec<bool>>) -> usize {
    fn has_a_contiguous_line(line: &[bool]) -> bool {
        line.iter().fold(0, |acc, value| {
            if acc >= LONG_LINE_THRESHOLD || *value {
                acc + 1
            } else {
                0
            }
        }) >= LONG_LINE_THRESHOLD
    }

    let mut all_lines = vec![];

    all_lines.extend(grid.iter().cloned());

    for y in 0..grid[0].len() {
        let mut row = vec![];

        for column in grid.iter() {
            row.push(column[y]);
        }

        all_lines.push(row);
    }

    all_lines
        .iter()
        .filter(|line| has_a_contiguous_line(line))
        .count()
}

const TOO_LARGE_WIDTH: i32 = 100;
const TOO_LARGE_HEIGHT: i32 = 100;

pub fn ten() -> u32 {
    let contents = fs::read_to_string("src/inputs/10.txt").unwrap();
    let points: Vec<Point> = contents.lines().map(Point::new).collect();

    let mut grid = Grid::new(points);

    let mut i = 0;

    loop {
        grid.advance();
        i += 1;

        if (grid.max_x - grid.min_x) > TOO_LARGE_WIDTH
            || (grid.max_y - grid.min_y) > TOO_LARGE_HEIGHT
        {
            continue;
        } else if num_lines(grid.to_vec()) > 8 {
            break;
        }
    }

    grid.print();

    i
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(ten(), 10355);
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
