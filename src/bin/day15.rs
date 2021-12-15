use advent_of_code_2021::aoc;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;

#[derive(Eq, PartialEq, Debug)]
struct State {
    cost: u32,
    position: aoc::Vec2,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a(grid: &Vec<Vec<u32>>) -> u32 {
    let (sizex, sizey) = (grid[0].len(), grid.len());
    let mut best = vec![vec![u32::MAX; sizex]; sizey];
    let mut heap = BinaryHeap::new();
    heap.push(State {
        cost: 0,
        position: aoc::Vec2 { x: 0, y: 0 },
    });
    while let Some(State { cost, position }) = heap.pop() {
        if position.y + 1 == grid.len() as i64 && position.x + 1 == grid[0].len() as i64 {
            return cost;
        }
        for neigh in position.neighbours() {
            if neigh.x >= 0
                && neigh.x < sizex as i64
                && neigh.y >= 0
                && neigh.y < sizey as i64
                && cost < best[neigh.y as usize][neigh.x as usize]
            {
                best[neigh.y as usize][neigh.x as usize] = cost;
                let new_cost = cost + grid[neigh.y as usize][neigh.x as usize];
                heap.push(State {
                    cost: new_cost,
                    position: neigh,
                })
            }
        }
    }
    0
}

fn b(grid: &Vec<Vec<u32>>) -> u32 {
    let (sizex, sizey) = (grid[0].len(), grid.len());
    let mut new_grid = vec![vec![0; sizex * 5]; sizey * 5];
    for y in 0..sizey {
        for x in 0..sizex {
            for yy in 0 as usize..5 {
                for xx in 0 as usize..5 {
                    let mut n = grid[y][x] + xx as u32 + yy as u32;
                    if n > 9 {
                        n -= 9;
                    }
                    new_grid[sizey * yy + y][sizex * xx + x] = n;
                }
            }
        }
    }
    a(&new_grid)
}

fn main() {
    let contents = fs::read_to_string("inputs/day15.txt").unwrap();
    let grid = contents
        .split("\n")
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    println!("Part 1: {}", a(&grid));
    println!("Part 2: {}", b(&grid));
}
