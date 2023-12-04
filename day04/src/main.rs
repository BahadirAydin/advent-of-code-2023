use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

lazy_static! {
    static ref RE_DECK: Regex = Regex::new(r"(\d)+").unwrap();
}

fn get_match_count(line: &str) -> u32 {
    let index = line.find(":").unwrap();
    let line_ = &line[index + 1..];
    let winner = line_.split("|").nth(0).unwrap();
    let my_numbers = line_.split("|").nth(1).unwrap();

    let my_numbers: Vec<u32> = RE_DECK
        .captures_iter(my_numbers)
        .map(|cap| cap[0].parse::<u32>().unwrap())
        .collect();
    RE_DECK
        .captures_iter(winner)
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
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
