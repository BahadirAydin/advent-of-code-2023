use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

fn determine_type(hand: &str) -> Type {
    let cards: Vec<char> = hand.chars().collect();
    let mut counts = HashMap::new();
    cards.iter().for_each(|c| {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    });
    match counts.len() {
        1 => Type::FiveOfKind,
        2 => {
            let mut values: Vec<i32> = counts.values().map(|v| *v).collect();
            values.sort();
            match values.last().unwrap() {
                4 => Type::FourOfKind,
                3 => Type::FullHouse,
                _ => panic!("Unexpected value"),
            }
        }
        3 => {
            let mut values: Vec<i32> = counts.values().map(|v| *v).collect();
            values.sort();
            match values.last().unwrap() {
                3 => Type::ThreeOfKind,
                2 => Type::TwoPair,
                _ => panic!("Unexpected value"),
            }
        }
        4 => Type::OnePair,
        5 => Type::HighCard,
        _ => panic!("Unexpected value"),
    }
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

fn joker(hand: &str) -> Type {
    let mut best_type = determine_type(hand);
    let max_count_char = hand
        .chars()
        .max_by_key(|c| hand.matches(*c).count())
        .unwrap();
    let new_hand = hand.replace(max_count_char, "J");
    let new_type = determine_type(&new_hand);
    if new_type > best_type {
        best_type = new_type;
    }
    best_type
}

fn sort(hand1: &str, hand2: &str, part2: bool) -> Ordering {
    let type1 = if part2 { joker(hand1) } else { determine_type(hand1) };
    let type2 = if part2 { joker(hand2) } else { determine_type(hand2) };
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
    println!("Part 1: {}", run(&mut game, false));
    println!("Part 2: {}", run(&mut game, true));
}
