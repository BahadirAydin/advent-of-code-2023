use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        let input =
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green; 8 blue , 8 red , 8 green";
        let game = match_info(input);
        assert_eq!(game.game_id, 1);
        assert_eq!(game.sets.len(), 4);
        assert_eq!(game.sets[0].blue, 3);
        assert_eq!(game.sets[0].red, 4);
        assert_eq!(game.sets[1].red, 1);
        assert_eq!(game.sets[1].green, 2);
        assert_eq!(game.sets[1].blue, 6);
        assert_eq!(game.sets[2].green, 2);
        assert_eq!(game.sets[3].blue, 8);
        assert_eq!(game.sets[3].red, 8);
        assert_eq!(game.sets[3].green, 8);
    }
}

struct RGB {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    game_id: u32,
    sets: Vec<RGB>,
}

lazy_static! {
    static ref RE_GAME_ID: Regex = Regex::new(r"Game (\d+):").unwrap();
    static ref RE_BLUE: Regex = Regex::new(r"(\d+) blue").unwrap();
    static ref RE_RED: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref RE_GREEN: Regex = Regex::new(r"(\d+) green").unwrap();
}

fn match_info(line: &str) -> Game {
    let game_id = RE_GAME_ID
        .captures(line)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .unwrap();

    let mut sets = Vec::new();
    for set in line.split(";") {
        let mut rgb = RGB {
            red: 0,
            green: 0,
            blue: 0,
        };
        if let Some(caps) = RE_BLUE.captures(set) {
            rgb.blue = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        }
        if let Some(caps) = RE_RED.captures(set) {
            rgb.red = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        }
        if let Some(caps) = RE_GREEN.captures(set) {
            rgb.green = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        }
        sets.push(rgb);
    }
    Game { game_id, sets }
}

fn is_valid(game: &Game, red: u32, green: u32, blue: u32) -> bool {
    game.sets
        .iter()
        .all(|set| set.red <= red && set.green <= green && set.blue <= blue)
}

// input values
const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match_info(line))
        .filter(|game| is_valid(game, RED, GREEN, BLUE))
        .map(|game| game.game_id)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| match_info(line))
        .map(|game| {
            let red = game.sets.iter().map(|s| s.red).max().unwrap();
            let green = game.sets.iter().map(|s| s.green).max().unwrap();
            let blue = game.sets.iter().map(|s| s.blue).max().unwrap();
            red * green * blue
        })
        .sum()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
