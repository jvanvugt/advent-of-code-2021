use advent_of_code_2021::aoc;
use std::fs;

fn flash(grid: &mut Vec<Vec<u8>>, to_flash: &mut Vec<aoc::Vec2>) -> usize {
    let mut num_flashes = 0;
    while to_flash.len() > 0 {
        let pos = to_flash.pop().unwrap();
        for neigh in pos.neighbours_diag() {
            if neigh.x < 0
                || neigh.x as usize >= grid[0].len()
                || neigh.y < 0
                || neigh.y as usize >= grid.len()
            {
                continue;
            }
            if grid[neigh.y as usize][neigh.x as usize] <= 9 {
                grid[neigh.y as usize][neigh.x as usize] += 1;
                if grid[neigh.y as usize][neigh.x as usize] > 9 {
                    to_flash.push(neigh);
                    num_flashes += 1
                }
            }
        }
    }
    num_flashes
}

fn increment_phase(grid: &mut Vec<Vec<u8>>) -> Vec<aoc::Vec2> {
    let mut to_flash = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x] += 1;
            if grid[y][x] > 9 {
                to_flash.push(aoc::Vec2 {
                    x: x as i64,
                    y: y as i64,
                });
            }
        }
    }
    to_flash
}

fn zero_flashed(grid: &mut Vec<Vec<u8>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] > 9 {
                grid[y][x] = 0;
            }
        }
    }
}

fn a(grid: &Vec<Vec<u8>>) -> usize {
    let mut total_flashes = 0;
    let mut grid = grid.clone();
    for _ in 0..100 {
        let mut to_flash = increment_phase(&mut grid);
        total_flashes += to_flash.len();
        total_flashes += flash(&mut grid, &mut to_flash);
        zero_flashed(&mut grid);
    }
    total_flashes
}

fn b(grid: &Vec<Vec<u8>>) -> usize {
    let mut grid = grid.clone();
    let grid_size = grid[0].len() * grid.len();
    for i in 1.. {
        let mut flashes = 0;
        let mut to_flash = increment_phase(&mut grid);
        flashes += to_flash.len();
        flashes += flash(&mut grid, &mut to_flash);
        zero_flashed(&mut grid);
        if flashes == grid_size {
            return i;
        }
    }
    return 0;
}

fn main() {
    let contents = fs::read_to_string("inputs/day11.txt").unwrap();
    let grid = contents
        .split("\n")
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect();
    println!("Part 1: {}", a(&grid));
    println!("Part 2: {}", b(&grid));
}
