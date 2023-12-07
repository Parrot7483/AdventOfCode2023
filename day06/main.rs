use itertools::Itertools;
use std::fs;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn wins(&self) -> u64 {
        let center = self.time / 2;
        let mut wins = 0;

        for i in (0..center).rev() {
            if (self.time - i) * i <= self.distance {
                break;
            }

            wins += 1;
        }

        for i in center..self.time {
            if (self.time - i) * i <= self.distance {
                break;
            }

            wins += 1;
        }

        wins
    }
}

fn parse1(input: &str) -> Vec<Race> {
    let (ts, ds) = input.lines().collect_tuple().unwrap();

    let times: Vec<u64> = ts[5..]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let distances: Vec<u64> = ds[9..]
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let races: Vec<_> = times
        .iter()
        .zip(distances)
        .map(|(t, d)| Race {
            time: *t,
            distance: d,
        })
        .collect();

    races
}

fn parse2(input: &str) -> Race {
    let (ts, ds) = input.lines().collect_tuple().unwrap();

    let time: u64 = ts
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    let distance: u64 = ds
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap();

    Race {
        time: time,
        distance: distance,
    }
}

fn part1(races: &Vec<Race>) -> u64 {
    races.iter().map(|r| r.wins()).product()
}

fn part2(race: &Race) -> u64 {
    race.wins()
}

fn main() {
    let file_path = "./day06/input.txt";
    let content = fs::read_to_string(file_path).unwrap();

    let races = parse1(&content);
    let race = parse2(&content);
    println!("Result part1: {}", part1(&races));
    println!("Result part2: {}", part2(&race));
}
