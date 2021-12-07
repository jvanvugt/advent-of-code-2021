use std::fs;

fn compute_cost(xs: &Vec<i64>, cost_fn: fn(i64) -> i64) -> i64 {
    let minx = xs.iter().min().unwrap();
    let maxx = xs.iter().max().unwrap();
    (*minx..=*maxx)
        .map(|i| xs.iter().map(|x| cost_fn(*x - i)).sum())
        .min()
        .unwrap()
}

fn main() {
    let contents = fs::read_to_string("inputs/day07.txt").unwrap();
    let crabs_pos: Vec<i64> = contents.split(",").map(|s| s.parse().unwrap()).collect();
    println!("Part 1: {}", compute_cost(&crabs_pos, |d| d.abs()));
    println!(
        "Part 2: {}",
        compute_cost(&crabs_pos, |d| (d.abs() * (d.abs() + 1)) / 2)
    );
}
