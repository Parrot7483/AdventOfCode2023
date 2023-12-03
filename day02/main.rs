use regex_lite::Regex;
use std::cmp::max;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct ParseGameError(String);

#[derive(Debug)]
struct ParseRoundError(String);

#[derive(Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Game {
    type Err = ParseGameError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Game ([[:digit:]]+): (.*)").unwrap();
        let captures = re.captures(s).unwrap();

        let id: u32 = captures.get(1).unwrap().as_str().parse().unwrap();

        let rounds: Vec<Round> = captures
            .get(2)
            .unwrap()
            .as_str()
            .split("; ")
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Game { id, rounds })
    }
}

impl FromStr for Round {
    type Err = ParseRoundError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re_red = Regex::new("([[:digit:]]+) red").unwrap();
        let re_blue = Regex::new("([[:digit:]]+) blue").unwrap();
        let re_green = Regex::new("([[:digit:]]+) green").unwrap();

        let red: u32 = match re_red.captures(s) {
            Some(num) => num.get(1).unwrap().as_str().parse().unwrap(),
            None => 0,
        };

        let green: u32 = match re_green.captures(s) {
            Some(num) => num.get(1).unwrap().as_str().parse().unwrap(),
            None => 0,
        };

        let blue: u32 = match re_blue.captures(s) {
            Some(num) => num.get(1).unwrap().as_str().parse().unwrap(),
            None => 0,
        };

        Ok(Round { red, green, blue })
    }
}

fn maxs(game: &Game) -> (u32, u32, u32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for round in &game.rounds {
        red = max(round.red, red);
        green = max(round.green, green);
        blue = max(round.blue, blue);
    }

    (red, green, blue)
}

fn part1(games: &Vec<Game>) -> u32 {
    let mut result = 0;

    for game in games {
        let (r, g, b) = maxs(&game);

        if r <= 12 && g <= 13 && b <= 14 {
            result += game.id;
        }
    }

    result
}

fn part2(games: &Vec<Game>) -> u32 {
    let mut result = 0;

    for game in games {
        let (r, g, b) = maxs(&game);
        let power = r * g * b;
        result += power
    }

    result
}

fn main() {
    let file_path = "./day02/input.txt";

    let games: Vec<Game> = read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Result part1: {}", part1(&games));
    println!("Result part2: {}", part2(&games));
}
