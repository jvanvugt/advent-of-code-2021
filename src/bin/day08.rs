use std::array::IntoIter;
use std::collections::HashMap;
use std::fs;

fn a(lines: &Vec<&str>) -> usize {
    let unique_counts: Vec<usize> = vec![2, 3, 4, 7];
    lines
        .iter()
        .map(|l| {
            l.split(" | ")
                .nth(1)
                .unwrap()
                .split(" ")
                .filter(|w| unique_counts.contains(&w.len()))
                .count()
        })
        .sum()
}

fn sorted_words(line: &str) -> Vec<String> {
    line.split(" ")
        .map(|w| {
            let mut word = w.chars().collect::<Vec<char>>();
            word.sort();
            String::from_iter(&word)
        })
        .collect()
}

fn get_output(line: &str) -> i32 {
    let (before, after) = line.split_once(" | ").unwrap();
    let mut before_words = sorted_words(before);
    let after_words = sorted_words(after);

    let mut letter_counts = HashMap::<char, usize>::new();
    for word in before_words.iter() {
        for letter in word.chars() {
            *letter_counts.entry(letter).or_insert(0) += 1;
        }
    }

    let one = before_words
        .iter()
        .filter(|w| w.len() == 2)
        .next()
        .unwrap()
        .clone();

    let mut assignment = HashMap::<_, _>::from_iter(IntoIter::new([
        (one.clone(), '1'),
        (
            before_words
                .iter()
                .filter(|w| w.len() == 4)
                .next()
                .unwrap()
                .clone(),
            '4',
        ),
        (
            before_words
                .iter()
                .filter(|w| w.len() == 3)
                .next()
                .unwrap()
                .clone(),
            '7',
        ),
        (
            before_words
                .iter()
                .filter(|w| w.len() == 7)
                .next()
                .unwrap()
                .clone(),
            '8',
        ),
    ]));

    assignment
        .keys()
        .for_each(|f| before_words.retain(|o| o != f));

    let bot_left_letter = letter_counts
        .iter()
        .filter(|(_, c)| **c == 4)
        .next()
        .unwrap()
        .0;
    let top_left_letter = letter_counts
        .iter()
        .filter(|(_, c)| **c == 6)
        .next()
        .unwrap()
        .0;

    let nine = before_words
        .iter()
        .filter(|s| !s.contains(*bot_left_letter) && s.len() == 6)
        .next()
        .unwrap()
        .clone();

    before_words.retain(|w| w != &nine);
    assignment.insert(nine, '9');

    let two = before_words
        .iter()
        .filter(|s| s.contains(*bot_left_letter) && s.len() == 5)
        .next()
        .unwrap()
        .clone();
    before_words.retain(|w| w != &two);
    assignment.insert(two, '2');

    let three = before_words
        .iter()
        .filter(|s| !s.contains(*top_left_letter) && s.len() == 5)
        .next()
        .unwrap()
        .clone();
    before_words.retain(|w| w != &three);
    assignment.insert(three, '3');

    let five = before_words
        .iter()
        .filter(|s| s.len() == 5)
        .next()
        .unwrap()
        .clone();
    before_words.retain(|w| w != &five);
    assignment.insert(five, '5');

    let zero = before_words
        .iter()
        .filter(|s| {
            s.contains(one.chars().nth(0).unwrap()) && s.contains(one.chars().nth(1).unwrap())
        })
        .next()
        .unwrap()
        .clone();
    before_words.retain(|w| w != &zero);
    assignment.insert(zero, '0');

    let six = before_words[0].clone();
    assignment.insert(six, '6');

    String::from_iter(after_words.iter().map(|w| assignment[w]))
        .parse()
        .unwrap()
}

fn b(lines: &Vec<&str>) -> i32 {
    lines.iter().map(|s| get_output(*s)).sum()
}

fn main() {
    let contents = fs::read_to_string("inputs/day08.txt").unwrap();
    let lines: Vec<&str> = contents.split("\n").collect();
    println!("Part 1: {}", a(&lines));
    println!("Part 2: {}", b(&lines));
}
