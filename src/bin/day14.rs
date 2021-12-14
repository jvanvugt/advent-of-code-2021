use std::{collections::HashMap, fs};

fn simulate(template: &Vec<char>, rules: &HashMap<(char, char), char>, num_iters: usize) -> u64 {
    let mut bigrams = HashMap::new();
    for (i, c) in template[0..template.len() - 1].iter().enumerate() {
        *bigrams.entry((*c, template[i + 1])).or_insert(0 as u64) += 1;
    }
    for _ in 0..num_iters {
        let mut updated = HashMap::new();
        for (pair, count) in bigrams.iter() {
            if let Some(new_char) = rules.get(&pair) {
                *updated.entry((pair.0, *new_char)).or_insert(0) += count;
                *updated.entry((*new_char, pair.1)).or_insert(0) += count;
            }
        }
        bigrams = updated;
    }
    let mut counts = HashMap::new();
    for ((c1, c2), count) in bigrams {
        *counts.entry(c1).or_insert(0 as u64) += count;
        *counts.entry(c2).or_insert(0 as u64) += count;
    }
    for v in counts.values_mut() {
        *v = *v / 2;
    }
    counts.entry(template[0]).and_modify(|c| *c += 1);
    counts
        .entry(template[template.len() - 1])
        .and_modify(|c| *c += 1);
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn main() {
    let contents = fs::read_to_string("inputs/day14.txt").unwrap();
    let (template, rules_s) = contents.split_once("\n\n").unwrap();
    let template = template.chars().collect();
    let rules = rules_s
        .split("\n")
        .map(|r| r.split_once(" -> ").unwrap())
        .map(|(s1, s2)| {
            (
                (s1.chars().nth(0).unwrap(), s1.chars().nth(1).unwrap()),
                s2.chars().next().unwrap(),
            )
        })
        .collect();
    println!("Part 1: {}", simulate(&template, &rules, 10));
    println!("Part 2: {}", simulate(&template, &rules, 40));
}
