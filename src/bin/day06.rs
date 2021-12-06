use std::fs;

fn calc_num_fish(rem_days: &Vec<i32>, num_days: usize) -> usize {
    // dp[i] := (num fish starting 7 day cyclus, num fish starting 9 day cyclus, total fish)
    let mut dp = vec![(0 as usize, 0 as usize, 0 as usize); num_days + 6];
    dp[0] = (1, 0, 1);
    for i in 1..dp.len() {
        let mut new_born: usize = 0;
        if i >= 9 {
            new_born += dp[i - 9].1;
        }
        if i >= 7 {
            new_born += dp[i - 7].0;
        }
        dp[i] = (new_born, new_born, dp[i - 1].2 + new_born);
    }
    rem_days.iter().map(|i| dp[dp.len() - *i as usize].2).sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/day06.txt").unwrap();
    let rem_days = contents
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    println!("{}", calc_num_fish(&rem_days, 80));
    println!("{}", calc_num_fish(&rem_days, 256));
}
