use std::fs;
use std::time::Instant;

const SIZE: usize = 140;

fn empty_rows(map: &[[char; SIZE]; SIZE]) -> Vec<usize> {
    map.iter()
        .enumerate()
        .filter(|(_, row)| !row.iter().any(|&c| c == '#'))
        .map(|(i, _)| i)
        .collect()
}
fn empty_cols(map: &[[char; SIZE]; SIZE]) -> Vec<usize> {
    (0..SIZE)
        .filter(|&j| !map.iter().any(|row| row[j] == '#'))
        .collect()
}

fn galaxy_positions(map: &[[char; SIZE]; SIZE]) -> Vec<(i64, i64)> {
    let mut positions = Vec::new();
    for i in 0..SIZE {
        for j in 0..SIZE {
            if map[i][j] == '#' {
                positions.push((i as i64, j as i64));
            }
        }
    }
    positions
}

fn run(map: &[[char; SIZE]; SIZE], part2: bool) -> i64 {
    let empty_rows = empty_rows(&map);
    let empty_cols = empty_cols(&map);
    let pos = galaxy_positions(&map);

    let mut sum = 0;
    for i in 0..pos.len() - 1 {
        for j in (i + 1)..pos.len() {
            let range_row = if pos[i].0 < pos[j].0 {
                pos[i].0..pos[j].0
            } else {
                pos[j].0..pos[i].0
            };
            let range_col = if pos[i].1 < pos[j].1 {
                pos[i].1..pos[j].1
            } else {
                pos[j].1..pos[i].1
            };
            sum += range_row.end - range_row.start + range_col.end - range_col.start;
            for col in empty_cols.iter() {
                if range_col.contains(&(*col as i64)) {
                    if part2 {
                        sum += 1000000 - 1;
                    } else {
                        sum += 1;
                    }
                }
            }
            for row in empty_rows.iter() {
                if range_row.contains(&(*row as i64)) {
                    if part2 {
                        sum += 1000000 - 1;
                    } else {
                        sum += 1;
                    }
                }
            }
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut map = [['.'; SIZE]; SIZE];
    input
        .lines()
        .enumerate()
        .for_each(|(i, line)| line.chars().enumerate().for_each(|(j, c)| map[i][j] = c));
    let start = Instant::now();
    let res1 = run(&map, false);
    let duration = start.elapsed();
    println!("Part 1: {} took {:#?}", res1, duration);
    let start = Instant::now();
    let res2 = run(&map, true);
    let duration = start.elapsed();
    println!("Part 2: {} took {:#?}", res2, duration);
}
