use advent_of_code_2021::aoc;
use std::{collections::HashSet, fs};

fn find_low_points(height_map: &Vec<Vec<u32>>) -> Vec<aoc::Vec2> {
    let mut result = Vec::<aoc::Vec2>::new();
    let size = aoc::Vec2 {
        x: height_map[0].len() as i64,
        y: height_map.len() as i64,
    };
    for y in 0..height_map.len() {
        for x in 0..height_map[y].len() {
            let pos = aoc::Vec2 {
                x: x as i64,
                y: y as i64,
            };
            let height = height_map[y][x];
            if pos.neighbours().iter().all(|n| {
                if n.x >= 0 && n.x < size.x && n.y >= 0 && n.y < size.y {
                    height < height_map[n.y as usize][n.x as usize]
                } else {
                    true
                }
            }) {
                result.push(pos);
            }
        }
    }
    result
}

fn a(height_map: &Vec<Vec<u32>>) -> u32 {
    find_low_points(height_map)
        .iter()
        .map(|p| height_map[p.y as usize][p.x as usize] + 1)
        .sum()
}

fn find_size(p: &aoc::Vec2, height_map: &Vec<Vec<u32>>) -> usize {
    let size = aoc::Vec2 {
        x: height_map[0].len() as i64,
        y: height_map.len() as i64,
    };
    let mut visited = HashSet::<aoc::Vec2>::new();
    visited.insert(p.clone());
    let mut todo = vec![p.clone()];
    while todo.len() > 0 {
        let cur = todo.pop().unwrap();
        let ch = height_map[cur.y as usize][cur.x as usize];
        for n in cur.neighbours() {
            if n.x >= 0 && n.x < size.x && n.y >= 0 && n.y < size.y {
                let nh = height_map[n.y as usize][n.x as usize];
                if !visited.contains(&n) && nh > ch && nh != 9 {
                    visited.insert(n.clone());
                    todo.push(n);
                }
            }
        }
    }
    visited.len()
}

fn b(height_map: &Vec<Vec<u32>>) -> usize {
    let mut basin_sizes: Vec<usize> = find_low_points(height_map)
        .iter()
        .map(|p| find_size(p, height_map))
        .collect();
    basin_sizes.sort_by(|a, b| b.cmp(a));
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn main() {
    let contents = fs::read_to_string("inputs/day09.txt").unwrap();
    let height_map: Vec<Vec<u32>> = contents
        .split("\n")
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    println!("Part 1: {}", a(&height_map));
    println!("Part 2: {}", b(&height_map));
}
