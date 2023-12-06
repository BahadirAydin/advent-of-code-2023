use regex::Regex;
use roots::find_roots_quadratic;
use roots::Roots;
use std::fs;

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    let re = Regex::new(r"(\d)+").unwrap();
    let mut lines = input.lines();
    let times = re
        .captures_iter(lines.next().unwrap())
        .map(|cap| cap[0].parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let distances = re
        .captures_iter(lines.next().unwrap())
        .map(|cap| cap[0].parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    (times, distances)
}

fn part1(times: Vec<i64>, distances: Vec<i64>) -> i64 {
    let mut res = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        // little value added becuase we need to break the record
        if let Roots::Two(values) =
            find_roots_quadratic(-1f64, *time as f64, (-*distance as f64) - 0.0001f64)
        {
            res *= values[1].floor() as i64 - values[0] as i64;
        }
    }
    res
}

fn part2(input: &str) -> i64 {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    part1(vec![time], vec![distance])
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let (times, distances) = parse(&input);
    println!("Part 1: {}", part1(times, distances));
    println!("Part 2: {}", part2(&input));
}
