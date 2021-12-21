use std::collections::HashMap;

fn a(start1: usize, start2: usize) -> usize {
    let mut pts1 = 0;
    let mut pts2 = 0;
    let mut roll: usize = 1;
    let mut pos1 = start1;
    let mut pos2 = start2;
    while std::cmp::max(pts1, pts2) < 1000 {
        let points = if roll % 6 < 3 { &mut pts1 } else { &mut pts2 };
        let pos = if roll % 6 < 3 { &mut pos1 } else { &mut pos2 };
        for _ in 0..3 {
            *pos = ((*pos + (roll - 1) % 100 + 1) - 1) % 10 + 1;
            roll += 1;
        }
        *points += *pos;
    }
    (roll - 1) * std::cmp::min(pts1, pts2)
}

const COUNTS_PER_ROLL: [(i32, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

type State = (i32, i32, bool, i32, i32);
fn _helper(state: State, cache: &mut HashMap<State, usize>) -> usize {
    let (pos_a, pos_b, turn_a, pts_a, pts_b) = state;
    if pts_a >= 21 {
        return 1;
    }
    if pts_b >= 21 {
        return 0;
    }
    if let Some(r) = cache.get(&state) {
        return *r;
    }

    let mut result = 0;
    let (pos, pts) = if turn_a {
        (pos_a, pts_a)
    } else {
        (pos_b, pts_b)
    };
    for (roll, count) in COUNTS_PER_ROLL.iter() {
        let new_pos = (pos + roll - 1) % 10 + 1;
        let new_pts = pts + new_pos;
        if turn_a {
            result += count * _helper((new_pos, pos_b, !turn_a, new_pts, pts_b), cache);
        } else {
            result += count * _helper((pos_a, new_pos, !turn_a, pts_a, new_pts), cache);
        }
    }
    cache.insert(state, result);
    result
}

fn b2(start_a: i32, start_b: i32) -> usize {
    let wins_a = _helper((start_a, start_b, true, 0, 0), &mut HashMap::new());
    let wins_b = _helper((start_b, start_a, false, 0, 0), &mut HashMap::new());
    std::cmp::max(wins_a, wins_b)
}

fn main() {
    let (s1, s2) = (2, 7);
    println!("Part 1: {}", a(s1, s2));
    println!("Part 2: {}", b2(s1 as i32, s2 as i32));
}
