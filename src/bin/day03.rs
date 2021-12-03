use std::{collections::HashSet, fs};

fn a(lines: &Vec<Vec<char>>) -> u64 {
    let num_bits = lines[0].len();
    let mut num_ones_per_idx = vec![0; num_bits];
    for bit_idx in 0..num_bits {
        for line in lines {
            num_ones_per_idx[bit_idx] += (line[bit_idx] == '0') as usize;
        }
    }
    let mut epsilon: u64 = 0;
    let mut gamma: u64 = 0;
    for (idx, count) in num_ones_per_idx.iter().enumerate() {
        let num = (*count > lines.len() / 2) as u64;
        gamma |= num << (num_ones_per_idx.len() - idx - 1);
        epsilon |= (1 - num) << (num_ones_per_idx.len() - idx - 1);
    }
    epsilon * gamma
}

fn b(lines: &Vec<Vec<char>>) -> u64 {
    let num_bits = lines[0].len();
    let mut possible_oxy = (0..lines.len()).collect::<HashSet<_>>();
    let mut possible_co2 = possible_oxy.clone();
    for bit_idx in 0..num_bits {
        let mut num_ones_oxy = 0;
        let mut num_ones_co2 = 0;
        for (line_idx, line) in lines.iter().enumerate() {
            if line[bit_idx] == '1' {
                if possible_oxy.contains(&line_idx) {
                    num_ones_oxy += 1;
                }
                if possible_co2.contains(&line_idx) {
                    num_ones_co2 += 1;
                }
            }
        }
        let expect_oxy =
            if num_ones_oxy > possible_oxy.len() / 2 || num_ones_oxy * 2 == possible_oxy.len() {
                '1'
            } else {
                '0'
            };
        let expect_co2 =
            if num_ones_co2 > possible_co2.len() / 2 || num_ones_co2 * 2 == possible_co2.len() {
                '0'
            } else {
                '1'
            };
        for (line_idx, line) in lines.iter().enumerate() {
            if possible_oxy.len() != 1
                && possible_oxy.contains(&line_idx)
                && line[bit_idx] != expect_oxy
            {
                possible_oxy.remove(&line_idx);
            }
            if possible_co2.len() != 1
                && possible_co2.contains(&line_idx)
                && line[bit_idx] != expect_co2
            {
                possible_co2.remove(&line_idx);
            }
        }
    }
    println!("{} {}", possible_co2.len(), possible_oxy.len());
    let oxy_bin = String::from_iter(&lines[*possible_oxy.iter().next().unwrap()]);
    let co2_bin = String::from_iter(&lines[*possible_co2.iter().next().unwrap()]);
    let oxy = u64::from_str_radix(&oxy_bin, 2).unwrap();
    let co2 = u64::from_str_radix(&co2_bin, 2).unwrap();
    oxy * co2
}

fn main() {
    let contents = fs::read_to_string("inputs/day03.txt").unwrap();
    let lines = contents
        .split("\n")
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("{}", a(&lines));
    println!("{}", b(&lines));
}
