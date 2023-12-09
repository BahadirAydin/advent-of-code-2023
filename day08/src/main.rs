use lazy_static::lazy_static;
use num::integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

lazy_static! {
    static ref RE: Regex = Regex::new(r"([A-Z]){3}").unwrap();
}

fn part1(instructions: &Vec<char>, map: &HashMap<&str, (&str, &str)>) -> i32 {
    let mut curr = "AAA";
    let mut pos = 0;
    let mut step = 0;

    while curr != "ZZZ" {
        let (a, b) = map.get(curr).unwrap();

        if instructions[pos] == 'R' {
            curr = b;
        } else {
            curr = a;
        }
        pos = (pos + 1) % instructions.len();
        step += 1;
    }

    step
}

fn part2(instructions: &Vec<char>, map: &HashMap<&str, (&str, &str)>) -> i64 {
    map.keys()
        .filter(|&k| k.ends_with('A'))
        .map(|&s| {
            let mut curr = s;
            let mut step = 0;
            let mut pos = 0;
            while !curr.ends_with('Z') {
                let (a, b) = map[curr];
                if instructions[pos] == 'R' {
                    curr = b;
                } else {
                    curr = a;
                }
                pos = (pos + 1) % instructions.len();
                step += 1;
            }
            step
        })
        .fold(1, lcm)
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut iter = input.lines();
    let instructions = iter.next().unwrap().chars().collect::<Vec<_>>();
    let mut map = HashMap::new();

    iter.skip(1).for_each(|line| {
        let caps = RE.captures_iter(line).collect::<Vec<_>>();
        map.insert(
            caps[0].get(0).unwrap().as_str(),
            (
                caps[1].get(0).unwrap().as_str(),
                caps[2].get(0).unwrap().as_str(),
            ),
        );
    });

    let start = Instant::now();
    let res1 = part1(&instructions, &map);
    let duration = start.elapsed();
    println!("Part 1: {} took {:#?}", res1, duration);
    let start = Instant::now();
    let res2 = part2(&instructions, &map);
    let duration = start.elapsed();
    println!("Part 2: {} took {:#?}", res2, duration);
}
