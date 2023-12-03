use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fs;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+)+").unwrap();
    static ref RE_SYM: Regex = Regex::new(r"([^\d.\n])+").unwrap();
}

fn part_numbers(input: &str, line_index: usize) -> u32 {
    let mut res = 0;
    let line = input.lines().nth(line_index).unwrap();
    for cap in RE.captures_iter(line) {
        let c = cap.get(1).unwrap();
        let start = c.start();
        let end = c.end();
        // positive lookbehind
        if start != 0 {
            let left = line.chars().nth(start - 1).unwrap();
            if left != '.' && !left.is_digit(10) {
                res += c.as_str().parse::<u32>().unwrap();
                continue;
            }
        }
        //positive lookahead
        if end != line.len() {
            let right: char = line.chars().nth(end).unwrap();
            if right != '.' && !right.is_digit(10) {
                res += c.as_str().parse::<u32>().unwrap();
                continue;
            }
        }
        // diagonal upper line
        if line_index != 0 {
            let upper_line = input.lines().nth(line_index - 1).unwrap();
            let n_start = if start == 0 { 0 } else { start - 1 };
            let n_end = cmp::min(end + 1, upper_line.len());
            if RE_SYM.is_match(&upper_line[n_start..n_end]) {
                res += c.as_str().parse::<u32>().unwrap();
                continue;
            }
        }
        // diagonal lower line
        if line_index != input.lines().count() - 1 {
            let lower_line = input.lines().nth(line_index + 1).unwrap();
            let n_start = if start == 0 { 0 } else { start - 1 };
            let n_end = cmp::min(end + 1, lower_line.len());
            if RE_SYM.is_match(&lower_line[n_start..n_end]) {
                res += c.as_str().parse::<u32>().unwrap();
                continue;
            }
        }
    }
    res
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .enumerate()
        .map(|(i, _)| part_numbers(input, i))
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
}
