use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::time::Instant;

lazy_static! {
    static ref RE_DECK: Regex = Regex::new(r"(\d)+").unwrap();
}

fn get_match_count(line: &str) -> u32 {
    let winner = line.split("|").nth(0).unwrap();
    let my_numbers = line.split("|").nth(1).unwrap();

    let my_numbers: Vec<u32> = RE_DECK
        .captures_iter(my_numbers)
        .map(|cap| cap[0].parse::<u32>().unwrap())
        .collect();
    RE_DECK
        .captures_iter(winner)
        .skip(1)
        .map(|cap| cap[0].parse::<u32>().unwrap())
        .filter(|x| my_numbers.contains(x))
        .count() as u32
}

fn winning_numbers(line: &str) -> u32 {
    let points = get_match_count(line);
    if points == 0 {
        return 0;
    }
    (2 as u32).pow(points - 1)
}
fn part1(input: &str) -> u32 {
    input.lines().map(|line| winning_numbers(line)).sum()
}

fn update_cards(card_count: &mut Vec<u32>, line: &str, current: usize) {
    let matches = get_match_count(line);
    for i in current + 1..current + 1 + matches as usize {
        card_count[i] += card_count[current];
    }
}

fn part2(input: &str) -> u32 {
    let mut card_count: Vec<u32> = vec![1; input.lines().count()];
    input.lines().enumerate().for_each(|(i, line)| {
        update_cards(&mut card_count, line, i);
    });
    card_count.iter().sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let start = Instant::now();
    let res1 = part1(&input);
    let duration = start.elapsed();
    println!("Part 1: {} took {:#?}", res1, duration);
    let start = Instant::now();
    let res2 = part2(&input);
    let duration = start.elapsed();
    println!("Part 2: {} took {:#?}", res2, duration);
}
