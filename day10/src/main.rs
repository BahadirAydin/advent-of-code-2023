use std::fs;
use std::time::Instant;
#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn determinant(&self, other: &Point) -> i32 {
        (self.x as i32) * (other.y as i32) - (self.y as i32) * (other.x as i32)
    }
}

const SIZE: usize = 140;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pipe {
    NE,
    NW,
    SE,
    SW,
    Horizontal,
    Vertical,
    Ground,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn from_point(&self, pos: &Point) -> Point {
        match self {
            Direction::North => Point {
                x: pos.x,
                y: pos.y - 1,
            },
            Direction::South => Point {
                x: pos.x,
                y: pos.y + 1,
            },
            Direction::East => Point {
                x: pos.x + 1,
                y: pos.y,
            },
            Direction::West => Point {
                x: pos.x - 1,
                y: pos.y,
            },
        }
    }
}

impl Pipe {
    fn possible_neighbors(&self) -> Vec<Direction> {
        match self {
            Pipe::NE => vec![Direction::North, Direction::East],
            Pipe::NW => vec![Direction::North, Direction::West],
            Pipe::SE => vec![Direction::South, Direction::East],
            Pipe::SW => vec![Direction::South, Direction::West],
            Pipe::Horizontal => vec![Direction::East, Direction::West],
            Pipe::Vertical => vec![Direction::North, Direction::South],
            _ => panic!("No neighbors for this pipe"),
        }
    }
}

type Map = [[Pipe; SIZE]; SIZE];

fn find_start_pipe(map: &Map, pos: &Point) -> Pipe {
    let down_pipe = if pos.y < SIZE - 1 {
        map[pos.y + 1][pos.x]
    } else {
        Pipe::Ground
    };
    let left_pipe = if pos.x > 0 {
        map[pos.y][pos.x - 1]
    } else {
        Pipe::Ground
    };
    if left_pipe == Pipe::SE && down_pipe == Pipe::NE {
        return Pipe::SW;
    }
    panic!("I only implemented case happening in my input");
}

fn run(map: &Map, start_position: Point) -> (i32, i32) {
    let pipe = find_start_pipe(map, &start_position);
    let possible_neighbors = pipe.possible_neighbors();
    let mut pos = start_position;
    let mut prev_pos = start_position;
    let mut steps = 1;
    let mut area = 0;
    for dir in possible_neighbors {
        let new_pos = dir.from_point(&start_position);
        if is_valid(&pos) {
            pos = new_pos;
            break;
        }
    }
    loop {
        let pipe = map[pos.y][pos.x];

        area += pos.determinant(&prev_pos);
        if pos == start_position {
            break;
        }
        let possible_neighbors = pipe.possible_neighbors();
        for dir in possible_neighbors {
            let new_pos = dir.from_point(&pos);
            if new_pos != prev_pos && is_valid(&new_pos) {
                prev_pos = pos;
                pos = new_pos;
                break;
            }
        }
        steps += 1;
    }
    (steps / 2, (area.abs() - steps) / 2 + 1)
}

fn is_valid(pos: &Point) -> bool {
    pos.x < SIZE && pos.y < SIZE
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start_position = input
        .iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.iter().enumerate().find_map(|(x, c)| {
                if *c == 'S' {
                    Some(Point { x, y })
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut map = [[Pipe::Ground; SIZE]; SIZE];
    input.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            map[y][x] = match c {
                '|' => Pipe::Vertical,
                '-' => Pipe::Horizontal,
                'L' => Pipe::NE,
                'J' => Pipe::NW,
                '7' => Pipe::SW,
                'F' => Pipe::SE,
                _ => Pipe::Ground,
            }
        })
    });

    let start = Instant::now();
    let res = run(&map, start_position);
    let duration = start.elapsed();
    println!(
        "Part 1: {} \nPart 2: {} \ntook {:?}",
        res.0, res.1, duration
    );
}
