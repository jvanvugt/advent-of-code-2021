use advent_of_code_2021::aoc;
use std::collections::HashMap;
use std::str::FromStr;
use std::{fs, string::ParseError};

struct Line {
    pub start: aoc::Vec2,
    pub end: aoc::Vec2,
}

impl FromStr for Line {
    type Err = ParseError;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let (from, to) = string.split_once(" -> ").unwrap();
        let (startx, starty) = from.split_once(",").unwrap();
        let (endx, endy) = to.split_once(",").unwrap();
        let start = aoc::Vec2 {
            x: startx.parse().unwrap(),
            y: starty.parse().unwrap(),
        };
        let end = aoc::Vec2 {
            x: endx.parse().unwrap(),
            y: endy.parse().unwrap(),
        };
        Ok(Line { start, end })
    }
}

fn count_intersections(lines: &Vec<Line>, with_diagonal: bool) -> usize {
    let mut intersections = HashMap::<aoc::Vec2, u64>::new();
    let low = aoc::Vec2 { x: -1, y: -1 };
    let high = aoc::Vec2 { x: 1, y: 1 };
    for line in lines {
        if line.start.x != line.end.x && line.start.y != line.end.y && !with_diagonal {
            continue;
        }
        let dir = (line.end - line.start).clamp(low, high);
        let mut pos = line.start;
        while pos != (line.end + dir) {
            *intersections.entry(pos).or_insert(0) += 1;
            pos = pos + dir;
        }
    }
    intersections.values().filter(|v| **v >= 2).count()
}

fn main() {
    let contents = fs::read_to_string("inputs/day05.txt").unwrap();
    let lines: Vec<Line> = contents.split("\n").map(|s| s.parse().unwrap()).collect();
    println!("Part 1: {}", count_intersections(&lines, false));
    println!("Part 2: {}", count_intersections(&lines, true));
}
