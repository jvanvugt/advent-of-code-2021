use std::fs;

#[derive(Clone, Copy, Debug)]
struct Field(i32, bool);
type Card = [[Field; 5]; 5];

fn sum_unmarked(card: &Card) -> i32 {
    card.iter()
        .flat_map(|c| c.iter())
        .filter(|f| !f.1)
        .map(|f| f.0)
        .sum()
}

fn mark_num_and_check_finished(num: i32, card: &mut Card) -> Option<i32> {
    for r in 0..card.len() {
        for c in 0..card[r].len() {
            if card[r][c].0 == num {
                card[r][c].1 = true;
                if (0..5).all(|r1| card[r1][c].1) || (0..5).all(|c1| card[r][c1].1) {
                    return Some(sum_unmarked(card) * num);
                }
            }
        }
    }
    None
}

fn play_game(nums: &Vec<i32>, cards: &Vec<Card>, return_first_win: bool) -> i32 {
    let mut score_last_win: i32 = 0;
    let mut cards = cards.clone();
    let mut active_cards = vec![true; cards.len()];
    for num in nums {
        for (card_idx, card) in cards.iter_mut().enumerate() {
            if !active_cards[card_idx] {
                continue;
            }
            match mark_num_and_check_finished(*num, card) {
                Some(res) => {
                    if return_first_win {
                        return res;
                    } else {
                        active_cards[card_idx] = false;
                        score_last_win = res;
                    }
                }
                None => {}
            }
        }
    }
    return score_last_win;
}

fn a(nums: &Vec<i32>, cards: &Vec<Card>) -> i32 {
    play_game(nums, cards, true)
}

fn b(nums: &Vec<i32>, cards: &Vec<Card>) -> i32 {
    play_game(nums, cards, false)
}

fn main() {
    let contents = fs::read_to_string("inputs/day04.txt").unwrap();
    let first_line = contents.split("\n").next().unwrap();
    let nums = first_line
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    let mut cards = Vec::<Card>::new();
    for card_string in contents.split("\n\n").skip(1) {
        let mut card: Card = [[Field(0, false); 5]; 5];
        for (card_line_idx, card_line_s) in card_string.split("\n").enumerate() {
            for (card_col_idx, num_s) in card_line_s.split_whitespace().enumerate() {
                card[card_line_idx][card_col_idx].0 = num_s.parse().unwrap();
            }
        }
        cards.push(card);
    }
    println!("{}", a(&nums, &cards));
    println!("{}", b(&nums, &cards));
}
