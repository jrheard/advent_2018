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
    width: usize,
    height: usize,
    buffer: Vec<Vec<bool>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Grid {
            width,
            height,
            buffer: vec![vec![false; height]; width],
        }
    }

    fn get(&self, x: i32, y: i32) -> bool {
        //println!("get {} {}", x, y);
        self.buffer[(x + (self.width as i32 / 2 + 1)) as usize]
            [(y + (self.height as i32 / 2 + 1)) as usize]
    }

    fn set(&mut self, x: i32, y: i32, value: bool) {
        //println!("set {} {}", x, y);
        self.buffer[(x + (self.width as i32 / 2 + 1)) as usize]
            [(y + (self.height as i32 / 2 + 1)) as usize] = value;
    }
}

pub fn ten_a() -> u32 {
    let contents = fs::read_to_string("src/inputs/10_sample.txt").unwrap();
    let points: Vec<Point> = contents.lines().map(Point::new).collect();

    let mut grid = Grid::new(51, 51);

    for point in &points {
        grid.set(point.x, point.y, true);
    }

    for y in -12..=12 {
        let mut row = String::new();

        for x in -12..=12 {
            row.push(if grid.get(x, y) { 'X' } else { '.' });
        }

        println!("{}", row);
    }

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
