use std::fs;

fn part1(input: &str) -> i32 {
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
fn part2(input: &str) -> i32 {
    let mut new_input = input.to_string();
    new_input = new_input.replace("one", "o1e");
    new_input = new_input.replace("two", "t2o");
    new_input = new_input.replace("three", "t3e");
    new_input = new_input.replace("four", "f4r");
    new_input = new_input.replace("five", "f5e");
    new_input = new_input.replace("six", "s6x");
    new_input = new_input.replace("seven", "s7n");
    new_input = new_input.replace("eight", "e8t");
    new_input = new_input.replace("nine", "n9e");

    part1(&new_input)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res1 = part1(&input);
    println!("Part 1: {}", res1);
    let res2 = part2(&input);
    println!("Part 2: {}", res2);
}
