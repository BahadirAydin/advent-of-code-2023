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

fn value_of_digit(digit: &str) -> i32 {
    match digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => unreachable!(),
    }
}

fn part2(input: &str) -> i32 {
    let digits_as_str = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut new_input = String::new();
    for line in input.lines() {
        let mut min_index = usize::MAX;
        let mut max_index = usize::MIN;
        let mut min_match = "";
        let mut max_match = "";
        let mut new_line = String::from(line) + "\n";
        for digit in digits_as_str.iter() {
            let index1 = match line.find(digit) {
                Some(i) => i,
                None => continue,
            };
            let index2 = match line.rfind(digit) {
                Some(i) => i,
                None => continue,
            };
            if index1 < min_index {
                min_index = index1;
                min_match = digit;
            }
            if index2 > max_index {
                max_index = index2;
                max_match = digit;
            }
            if min_index == usize::MAX {
                continue;
            } else if max_index == min_index {
                new_line = format!(
                    "{}{}{}\n",
                    line[..min_index].to_string(),
                    value_of_digit(min_match),
                    line[min_index..].to_string()
                );
            } else {
                new_line = format!(
                    "{}{}{}{}{}\n",
                    line[..min_index].to_string(),
                    value_of_digit(min_match),
                    line[min_index..max_index].to_string(),
                    value_of_digit(max_match),
                    line[max_index..].to_string()
                );
            }
        }
        new_input.push_str(&new_line);
    }
    part1(&new_input)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let res1 = part1(&input);
    println!("Part 1: {}", res1);
    let res2 = part2(&input);
    println!("Part 2: {}", res2);
}
