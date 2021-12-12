use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

struct State<'a>(&'a str, HashSet<&'a str>);

fn a(edges: &HashMap<&str, Vec<&str>>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front(State(
        "start",
        HashSet::from_iter(vec!["start"].iter().cloned()),
    ));
    let mut num_paths = 0;
    while queue.len() > 0 {
        let State(pos, visited) = queue.pop_back().unwrap();
        let neighbours = &edges[pos];
        for neigh in neighbours.iter() {
            let mut v = visited.clone();
            if *neigh == "end" {
                num_paths += 1;
            } else {
                if *neigh == neigh.to_lowercase() {
                    if v.contains(neigh) {
                        continue;
                    } else {
                        v.insert(neigh);
                    }
                }
                queue.push_front(State(*neigh, v));
            }
        }
    }
    num_paths
}

struct State2<'a>(&'a str, HashMap<&'a str, usize>);
fn b(edges: &HashMap<&str, Vec<&str>>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_front(State2("start", HashMap::new()));
    let mut num_paths = 0;
    while queue.len() > 0 {
        let State2(pos, visited) = queue.pop_back().unwrap();
        let neighbours = &edges[pos];
        for neigh in neighbours.iter() {
            let mut v = visited.clone();
            if *neigh == "end" {
                num_paths += 1;
            } else {
                if *neigh == neigh.to_lowercase() {
                    if (v.contains_key(neigh) && v.values().any(|c| *c == 2)) || *neigh == "start" {
                        continue;
                    } else {
                        *v.entry(neigh).or_insert(0) += 1;
                    }
                }
                queue.push_front(State2(*neigh, v));
            }
        }
    }
    num_paths
}

fn main() {
    let mut edges = HashMap::new();
    let contents = fs::read_to_string("inputs/day12.txt").unwrap();
    for line in contents.split("\n") {
        let (a, b) = line.split_once("-").unwrap();
        edges.entry(a).or_insert(Vec::new()).push(b);
        edges.entry(b).or_insert(Vec::new()).push(a);
    }
    println!("Part 1: {}", a(&edges));
    println!("Part 2: {}", b(&edges));
}
