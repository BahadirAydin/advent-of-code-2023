use std::cmp::Ordering;
use std::fs;
use std::time::Instant;


fn card_strength(hand: &str) -> i32 {
    let mut cards: Vec<char> = hand.chars().collect();
    cards.sort_unstable();
    let mut counts : Vec<(char, i32)> = vec![];
    cards.iter().for_each(|c| {
        if counts.len() == 0 || counts.last().unwrap().0 != *c {
            counts.push((*c, 1));
        } else {
            counts.last_mut().unwrap().1 += 1;
        }
    });
    counts.sort_unstable_by(|(_, c1), (_, c2)| c2.cmp(c1));
    counts.iter().enumerate()
        .map(|(i,(_,v))| v*(5 - i as i32 ) )
        .sum::<i32>()
}

fn char_strength(c: char, p2: bool) -> i32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => { if p2 { 1 } else { 11 }}
        'T' => 10,
        _ => c.to_digit(10).unwrap() as i32,
    }
}

fn joker(hand: &str) -> i32 {
    let mut best_type = card_strength(hand);
    let max_count_char = hand
        .chars()
        .max_by_key(|c| hand.matches(*c).count())
        .unwrap();
    let new_hand = hand.replace(max_count_char, "J");
    let new_type = card_strength(&new_hand);
    if new_type > best_type {
        best_type = new_type;
    }
    best_type
}

fn sort(hand1: &str, hand2: &str, part2: bool) -> Ordering {
    let type1 = if part2 { joker(hand1) } else { card_strength(hand1) };
    let type2 = if part2 { joker(hand2) } else { card_strength(hand2) };
    if type1.cmp(&type2) != Ordering::Equal {
        type1.cmp(&type2)
    } else {
        for (c1, c2) in hand1.chars().zip(hand2.chars()) {
            let s1 = char_strength(c1, part2);
            let s2 = char_strength(c2, part2);
            if s1 != s2 {
                return s1.cmp(&s2);
            }
        }
        panic!("Hands cannot tie.");
    }
}

fn run(game: &mut Vec<(&str, i32)>, part2: bool) -> i32 {
    let mut sum = 0;
    game.sort_unstable_by(|(hand1, _), (hand2, _)| sort(hand1, hand2, part2));
    for (i, (_, bid)) in game.iter().enumerate() {
        sum += (i as i32 + 1) * bid;
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut game: Vec<(&str, i32)> = input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_at(6);
            (hand.trim(), bid.parse::<i32>().unwrap())
        })
        .collect();
    let start = Instant::now();
    let res1 = run(&mut game, false);
    let duration = start.elapsed();
    println!("Part 1: {} took {:#?}", res1, duration);
    let start = Instant::now();
    let res2 = run(&mut game, true);
    let duration = start.elapsed();
    println!("Part 2: {} took {:#?}", res2, duration);
}
