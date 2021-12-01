use std::fs;

fn a(nums: &Vec<i32>) -> i32 {
    nums.iter()
        .zip(nums[1..].iter())
        .fold(0, |acc, (c, n)| acc + (c < n) as i32)
}

fn b(nums: &Vec<i32>) -> i32 {
    let mut cumsum = vec![0];
    for num in nums.iter() {
        cumsum.push(cumsum.last().unwrap() + num);
    }
    let k = 3;
    let windows: Vec<i32> = (0..cumsum.len() - k)
        .map(|i| cumsum[i + k] - cumsum[i])
        .collect();
    a(&windows)
}

fn main() {
    let contents = fs::read_to_string("inputs/day01.txt").unwrap();
    let nums: Vec<i32> = contents
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    println!("{}", a(&nums));
    println!("{}", b(&nums));
}
