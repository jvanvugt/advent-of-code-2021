use advent_of_code_2021::aoc;
use std::fs;

struct Command<'a>(&'a str, i64);

fn a(commands: &Vec<Command>) -> i64 {
    let mut pos = aoc::Vec2 { x: 0, y: 0 };
    for command in commands {
        match command.0 {
            "forward" => pos.x += command.1,
            "down" => pos.y += command.1,
            "up" => pos.y -= command.1,
            _ => panic!("{}", command.0),
        }
    }
    pos.x * pos.y
}

fn b(commands: &Vec<Command>) -> i64 {
    let mut pos = aoc::Vec2 { x: 0, y: 0 };
    let mut aim = 0;
    for command in commands {
        match command.0 {
            "forward" => {
                pos.x += command.1;
                pos.y += aim * command.1;
            }
            "down" => aim += command.1,
            "up" => aim -= command.1,
            _ => panic!("{}", command.0),
        }
    }
    pos.x * pos.y
}

fn main() {
    let contents = fs::read_to_string("inputs/day02.txt").unwrap();
    let commands = contents
        .split("\n")
        .map(|s| {
            let parts = s.split(" ").collect::<Vec<_>>();
            Command(parts[0], parts[1].parse().unwrap())
        })
        .collect::<Vec<_>>();
    println!("{}", a(&commands));
    println!("{}", b(&commands));
}
