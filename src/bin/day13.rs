use std::{collections::HashSet, fs};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Point(i32, i32);
struct Fold(char, i32);

fn do_fold(points: &HashSet<Point>, fold: &Fold) -> HashSet<Point> {
    let mut new_points = HashSet::new();
    for point in points {
        if fold.0 == 'x' && point.0 > fold.1 {
            new_points.insert(Point(fold.1 + fold.1 - point.0, point.1));
        } else if fold.0 == 'y' && point.1 > fold.1 {
            new_points.insert(Point(point.0, fold.1 + fold.1 - point.1));
        } else {
            new_points.insert(point.clone());
        }
    }
    new_points
}

fn a(points: &HashSet<Point>, folds: &Vec<Fold>) -> usize {
    do_fold(points, &folds[0]).len()
}

fn b(points: &HashSet<Point>, folds: &Vec<Fold>) {
    let mut points = points.clone();
    for fold in folds {
        points = do_fold(&points, fold);
    }
    let maxx = points.iter().map(|p| p.0).max().unwrap();
    let maxy = points.iter().map(|p| p.1).max().unwrap();
    let mut display =
        vec![String::from_utf8(vec![b'.'; maxx as usize + 1]).unwrap(); maxy as usize + 1];
    for point in points {
        display[point.1 as usize].replace_range(point.0 as usize..point.0 as usize + 1, "#");
    }
    for line in display {
        println!("{}", line);
    }
}

fn main() {
    let contents = fs::read_to_string("inputs/day13.txt").unwrap();
    let (points_str, folds_str) = contents.split_once("\n\n").unwrap();
    let points = points_str
        .split("\n")
        .map(|l| l.split_once(",").unwrap())
        .map(|(l, r)| Point(l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    let folds = folds_str
        .split("\n")
        .map(|l| l.split_once("=").unwrap())
        .map(|(l, r)| Fold(l.chars().last().unwrap(), r.parse().unwrap()))
        .collect();
    println!("Part 1: {}", a(&points, &folds));
    b(&points, &folds);
}
