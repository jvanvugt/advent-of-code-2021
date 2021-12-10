use std::fs;

fn validate_line(line: &str) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();
    let opening = vec!['(', '{', '[', '<'];
    for c in line.chars() {
        if opening.contains(&c) {
            stack.push(match c {
                '(' => ')',
                '{' => '}',
                '<' => '>',
                '[' => ']',
                _ => panic!("{}", c),
            });
        } else {
            let expected = stack.pop().unwrap();
            if c != expected {
                return Err(c);
            }
        }
    }
    Ok(stack.into_iter().rev().collect())
}

fn error_score_char(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("{}", c),
    }
}

fn autocomplete_score(chars: &Vec<char>) -> usize {
    let mut result = 0;
    for c in chars.into_iter() {
        result *= 5;
        result += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("{}", c),
        }
    }
    return result;
}

fn a(lines: &Vec<&str>) -> usize {
    lines
        .iter()
        .map(|l| validate_line(l))
        .map(|r| match r {
            Ok(_) => 0,
            Err(c) => error_score_char(c),
        })
        .sum()
}

fn b(lines: &Vec<&str>) -> usize {
    let mut scores: Vec<usize> = lines
        .iter()
        .map(|l| validate_line(l))
        .filter(|r| r.is_ok())
        .map(|r| match r {
            Ok(to_finish) => autocomplete_score(&to_finish),
            Err(_) => 0,
        })
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let contents = fs::read_to_string("inputs/day10.txt").unwrap();
    let lines = contents.split("\n").collect();
    println!("Part 1: {}", a(&lines));
    println!("Part 2: {}", b(&lines));
}
