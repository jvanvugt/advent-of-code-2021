use std::cmp;
use std::fs;
use std::ops::RangeInclusive;

fn solve(targetx: &RangeInclusive<i32>, targety: &RangeInclusive<i32>) -> (i32, i32) {
    let mut highest_y = 0;
    let range: i32 = 1000;
    let mut total_hit = 0;
    for y_vel in -range..range {
        for x_vel in -range..range {
            let mut y_vel = y_vel;
            let mut x_vel = x_vel;
            let mut x = 0;
            let mut y = 0;
            let mut throw_highest_y = 0;
            while y_vel >= 0 || y >= *targety.start() {
                x += x_vel;
                y += y_vel;
                throw_highest_y = cmp::max(throw_highest_y, y);
                x_vel -= x_vel.signum();
                y_vel -= 1;
                if targetx.contains(&x) && targety.contains(&y) {
                    highest_y = cmp::max(throw_highest_y, highest_y);
                    total_hit += 1;
                    break;
                }
            }
        }
    }
    return (highest_y, total_hit);
}

fn main() {
    let contents = fs::read_to_string("inputs/day17.txt").unwrap();
    let (fromx, tox) = contents
        .split_once("x=")
        .unwrap()
        .1
        .split_once(",")
        .unwrap()
        .0
        .split_once("..")
        .unwrap();
    let (fromy, toy) = contents
        .split_once("y=")
        .unwrap()
        .1
        .split_once("..")
        .unwrap();
    let x_range = fromx.parse().unwrap()..=tox.parse().unwrap();
    let y_range = fromy.parse().unwrap()..=toy.parse().unwrap();
    let (answer1, answer2) = solve(&x_range, &y_range);
    println!("Part 1: {}", answer1);
    println!("Part 2: {}", answer2);
}
