use regex_lite::Regex;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct ParseCardError(String);

#[derive(Debug, Hash, PartialEq, Eq)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    have: Vec<u32>,
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Card\s+(\d+): (.*) \| (.*)").unwrap();
        let captures = re.captures(s).unwrap();

        let id: u32 = captures.get(1).unwrap().as_str().parse().unwrap();

        let winning: Vec<u32> = captures
            .get(2)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        let have: Vec<u32> = captures
            .get(3)
            .unwrap()
            .as_str()
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        Ok(Card { id, winning, have })
    }
}

fn intersection(card: &Card) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    for i in &card.have {
        if card.winning.contains(&i) {
            result.push(*i);
        }
    }

    result
}

fn part1(cards: &Vec<Card>) -> u32 {
    let mut result: u32 = 0;
    for c in cards {
        let common = intersection(&c).len() as u32;
        if common > 0 {
            result += u32::pow(2, common - 1);
        }
    }

    result
}

fn part2(cards: &Vec<Card>) -> u32 {
    let mut card_count: Vec<u32> = vec![1; cards.len()];

    for c in cards {
        let common = intersection(c).len();

        for i in 1..(common + 1) {
            card_count[(c.id - 1) as usize + i] += card_count[(c.id - 1) as usize];
        }
    }

    card_count.iter().sum()
}

fn main() {
    let file_path = "./day04/input.txt";
    let cards: Vec<Card> = fs::read_to_string(file_path)
        .unwrap()
        .lines()
        .map(|c| c.parse().unwrap())
        .collect();

    println!("Result part1: {}", part1(&cards));
    println!("Result part2: {}", part2(&cards));
}
