use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::time::Instant;

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d)+").unwrap();
}

#[derive(Clone, Copy, Debug)]
struct Map {
    destination_start: i64,
    source_start: i64,
    range: i64,
}

type Seeds = Vec<i64>;

struct Parsed {
    seeds: Seeds,
    maps: Vec<Vec<Map>>,
}

fn parse_input(input: &str) -> Parsed {
    let mut lines = input.lines();
    let seeds_line = lines.next().unwrap();
    let seeds = RE
        .captures_iter(seeds_line)
        .map(|cap| cap[0].parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut maps: Vec<Vec<Map>> = Vec::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            lines.next();
            maps.push(Vec::new());
            continue;
        }
        let v = RE
            .captures_iter(line)
            .map(|cap| cap[0].parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        maps.last_mut().unwrap().push(Map {
            destination_start: v[0],
            source_start: v[1],
            range: v[2],
        });
    }
    Parsed { seeds, maps }
}

fn part1(input: &str) -> i64 {
    let parsed = parse_input(input);
    let mut min = i64::MAX;
    for seed in parsed.seeds.iter() {
        let mut t = *seed;
        for map in parsed.maps.iter() {
            for m in map.iter() {
                if m.source_start <= t && m.source_start + m.range >= t {
                    t = m.destination_start + t - m.source_start;
                    break;
                }
            }
        }
        if t < min {
            min = t;
        }
    }
    min
}

fn intersect_range(a: (i64, i64), b: (i64, i64)) -> Option<(i64, i64)> {
    let (a_start, a_end) = a;
    let (b_start, b_end) = b;
    if a_start <= b_start && a_end >= b_start {
        Some((b_start, a_end.min(b_end)))
    } else if b_start <= a_start && b_end >= a_start {
        Some((a_start, a_end.min(b_end)))
    } else {
        None
    }
}

fn propagate_ranges(mut ranges: Vec<(i64, i64)>, maps: &[Map]) -> Vec<(i64, i64)> {
    let mut i = 0;
    let mut new_ranges = Vec::new();
    while i < ranges.len() {
        let mut found = false;
        for map in maps.iter() {
            if let Some((start, end)) = intersect_range(
                (map.source_start, map.source_start + map.range - 1),
                ranges[i],
            ) {
                found = true;
                if ranges[i].0 < start {
                    ranges.push((ranges[i].0, start - 1));
                }
                if ranges[i].1 > end {
                    ranges.push((end + 1, ranges[i].1));
                }
                let offset = map.destination_start - map.source_start;
                new_ranges.push((start + offset, end + offset));
            }
        }
        if found {
            ranges.remove(i);
        } else {
            i += 1;
        }
    }
    ranges.iter().for_each(|r| new_ranges.push(*r));

    new_ranges
}

fn part2(input: &str) -> i64 {
    let parsed = parse_input(input);
    let layers = parsed.maps;

    // Create initial ranges from seeds
    let ranges = parsed
        .seeds
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1] - 1))
        .collect::<Vec<_>>();

    let mut current_ranges = ranges;
    for layer in layers.iter() {
        current_ranges = propagate_ranges(current_ranges, layer);
    }

    *(current_ranges.iter().map(|(start, _)| start).min().unwrap())
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let start = Instant::now();
    let res1 = part1(&input);
    let duration = start.elapsed();
    println!("Part 1: {} took {:#?}", res1, duration);
    let start = Instant::now();
    let res2 = part2(&input);
    let duration = start.elapsed();
    println!("Part 2: {} took {:#?}", res2, duration);
}
