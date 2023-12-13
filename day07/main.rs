use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Eq, Hash, Copy, Clone, PartialEq, PartialOrd)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, Eq, Hash, Copy, Clone, PartialOrd, PartialEq)]
struct Hand(Card, Card, Card, Card, Card);

#[derive(Debug, PartialEq)]
enum Strength {
    FiveOfKind = 6,
    FourOfKind = 5,
    FullHouse = 4,
    ThreeOfKind = 3,
    TwoPairs = 2,
    OnePair = 1,
    HighCard = 0,
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            x => panic!("Found unknown card: '{x}'"),
        }
    }
}

impl Card {
    fn _cmp(&self, other: &Self, ranks: &HashMap<Card, u8>) -> Ordering {
        if ranks.get(self) == ranks.get(other) {
            Ordering::Equal
        } else if ranks.get(self) > ranks.get(other) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }

    fn cmp1(&self, other: &Self) -> Ordering {
        let ranks: HashMap<Card, u8> = [
            (Card::Ace, 14),
            (Card::King, 13),
            (Card::Queen, 12),
            (Card::Jack, 11),
            (Card::Ten, 10),
            (Card::Nine, 9),
            (Card::Eight, 8),
            (Card::Seven, 7),
            (Card::Six, 6),
            (Card::Five, 5),
            (Card::Four, 4),
            (Card::Three, 3),
            (Card::Two, 2),
        ]
        .iter()
        .cloned()
        .collect();

        self._cmp(other, &ranks)
    }

    fn cmp2(&self, other: &Self) -> Ordering {
        let ranks: HashMap<Card, u8> = [
            (Card::Ace, 14),
            (Card::King, 13),
            (Card::Queen, 12),
            (Card::Ten, 11),
            (Card::Nine, 10),
            (Card::Eight, 9),
            (Card::Seven, 8),
            (Card::Six, 7),
            (Card::Five, 6),
            (Card::Four, 5),
            (Card::Three, 4),
            (Card::Two, 3),
            (Card::Jack, 2),
        ]
        .iter()
        .cloned()
        .collect();

        self._cmp(other, &ranks)
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b, c, d, e) = s.chars().map(|c| c.into()).collect_tuple().unwrap();
        Ok(Hand(a, b, c, d, e))
    }
}

impl Hand {
    fn strength(&self) -> Strength {
        let mut counts = HashMap::new();

        for i in [self.0, self.1, self.2, self.3, self.4].iter() {
            *counts.entry(*i).or_insert(0) += 1
        }

        if counts.len() == 1 {
            return Strength::FiveOfKind;
        } else if counts.values().contains(&4) {
            return Strength::FourOfKind;
        } else if counts.values().contains(&3) && counts.len() == 2 {
            return Strength::FullHouse;
        } else if counts.values().contains(&3) && counts.len() == 3 {
            return Strength::ThreeOfKind;
        } else if counts.values().filter(|c| **c == 2).count() == 2 {
            return Strength::TwoPairs;
        } else if counts.len() == 4 {
            return Strength::OnePair;
        } else {
            return Strength::HighCard;
        }
    }

    fn change(&self, i: usize, c: Card) -> Hand {
        let mut hand = self.clone();
        match i {
            0 => hand.0 = c,
            1 => hand.1 = c,
            2 => hand.2 = c,
            3 => hand.3 = c,
            4 => hand.4 = c,
            _ => panic!("Can not set {i}'th Card in Hand"),
        }
        hand
    }

    fn gen_hands(&self) -> Vec<Hand> {
        if !self.to_vec().contains(&Card::Jack) {
            return vec![self.clone()];
        }

        let mut result = Vec::new();
        for (i, card) in self.to_vec().into_iter().enumerate() {
            if card == Card::Jack {
                for new in [
                    Card::Two,
                    Card::Three,
                    Card::Four,
                    Card::Five,
                    Card::Six,
                    Card::Seven,
                    Card::Eight,
                    Card::Nine,
                    Card::Ten,
                    Card::Queen,
                    Card::King,
                    Card::Ace,
                ] {
                    result.extend(self.change(i, new).gen_hands());
                }
            }
        }
        result
    }

    fn best(&self) -> Hand {
        *self.gen_hands().iter().max_by(|a, b| a.cmp1(&b)).unwrap()
    }

    fn cmp1(&self, other: &Self) -> Ordering {
        let l = self.strength() as u8;
        let r = other.strength() as u8;

        if self == other {
            return Ordering::Equal;
        } else if l > r {
            return Ordering::Greater;
        } else if l < r {
            return Ordering::Less;
        }

        for (s, o) in self.to_vec().iter().zip(other.to_vec().iter()) {
            let r = s.cmp1(&o);
            if r != Ordering::Equal {
                return r;
            }
        }

        Ordering::Equal
    }

    fn to_vec(&self) -> Vec<Card> {
        [self.0, self.1, self.2, self.3, self.4].to_vec()
    }
}

fn cmp((h1, b1): (&Hand, &Hand), (h2, b2): (&Hand, &Hand)) -> Ordering {
    let s1 = b1.strength() as u8;
    let s2 = b2.strength() as u8;

    if s1 > s2 {
        return Ordering::Greater;
    } else if s1 < s2 {
        return Ordering::Less;
    }

    for (c1, c2) in h1.to_vec().iter().zip(h2.to_vec().iter()) {
        let r = c1.cmp2(&c2);
        if r != Ordering::Equal {
            return r;
        }
    }

    Ordering::Equal
}

fn part1(h: &Vec<(Hand, u64)>) -> u64 {
    let mut hands = h.to_vec();
    hands.sort_by(|(a, _), (b, _)| a.cmp1(&b));
    hands.iter().zip(1..).map(|((_, v), i)| v * i).sum()
}

fn part2(h: &Vec<(Hand, u64)>) -> u64 {
    let mut hands: Vec<_> = h.iter().map(|(h, v)| (h, v, h.best())).collect();
    hands.sort_by(|(h1, _, b1), (h2, _, b2)| cmp((&h1, &b1), (&h2, &b2)));
    hands.iter().zip(1..).map(|((_, v, _), i)| *v * i).sum()
}

fn parse(s: &str) -> Vec<(Hand, u64)> {
    s.lines()
        .map(|l| l.split_whitespace().collect_tuple().unwrap())
        .map(|(h, b)| (h.parse().unwrap(), b.parse().unwrap()))
        .collect()
}

fn main() {
    let file_path = "./day07/input.txt";
    let content = fs::read_to_string(file_path).unwrap();

    let hands = parse(&content);

    println!("Result 1: {}", part1(&hands));
    println!("Result 2: {}", part2(&hands));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let content = fs::read_to_string("./day07/input.txt").unwrap();
        assert_eq!(246795406, part1(&parse(&content)));
    }

    #[test]
    fn test_part2() {
        let content = fs::read_to_string("./day07/input.txt").unwrap();
        assert_eq!(249356515, part2(&parse(&content)));
    }
}
