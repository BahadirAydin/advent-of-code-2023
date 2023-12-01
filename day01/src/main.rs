use std::fs;

fn part1() -> i32 {
    let input = fs::read_to_string("input.txt").unwrap();
    let first_digits: Vec<i32> = input
        .lines()
        .filter_map(|line| {
            line.chars()
                .find(|c| c.is_digit(10))
                .map(|digit_char| digit_char.to_digit(10).unwrap() as i32)
        })
        .collect();

    let second_digits: Vec<i32> = input
        .lines()
        .filter_map(|line| {
            line.chars()
                .rev()
                .find(|c| c.is_digit(10))
                .map(|digit_char| digit_char.to_digit(10).unwrap() as i32)
        })
        .collect();

    second_digits
        .iter()
        .zip(first_digits.iter())
        .map(|(a, b)| 10 * b + a)
        .sum()
}

fn main() {
    let res1 = part1();
    println!("Part 1: {}", res1);
}
