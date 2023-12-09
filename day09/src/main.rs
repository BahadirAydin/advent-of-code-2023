use std::fs;

fn go_down(arr: &Vec<i32>) -> Vec<i32> {
    arr.iter()
        .zip(arr.iter().skip(1))
        .map(|(x1, x2)| x2 - x1)
        .collect()
}

fn part1(numbers: &Vec<Vec<i32>>) -> i32 {
    numbers
        .iter()
        .map(|arr| {
            let mut curr = arr.clone();
            let mut last_elements = vec![];
            while curr.iter().any(|x| *x != 0) {
                last_elements.push(curr[curr.len() - 1]);
                curr = go_down(&curr);
            }
            last_elements.iter().fold(0, |acc, x| acc + x)
        })
        .sum()
}

fn part2(numbers: &Vec<Vec<i32>>) -> i32 {
    numbers
        .iter()
        .map(|arr| {
            let mut curr = arr.clone();
            let mut first_elements = vec![];
            while curr.iter().any(|x| *x != 0) {
                first_elements.push(curr[0]);
                curr = go_down(&curr);
            }
            first_elements.iter().rev().fold(0, |acc, x| x - acc)
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let numbers: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();
    println!("Part 1: {}", part1(&numbers));
    println!("Part 2: {}", part2(&numbers));
}
