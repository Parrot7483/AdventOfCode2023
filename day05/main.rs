// use regex_lite::Regex;
use itertools::Itertools;
use std::fs;

#[derive(Debug)]
struct Recipe {
    maps: Vec<(u64, u64, u64)>,
}

impl Recipe {
    fn map(&self, i: u64) -> u64 {
        for (d, s, l) in &self.maps {
            if i >= *s && i < *s + *l {
                return (i - *s) + *d;
            }
        }

        i
    }
}

fn apply_recipes(n: u64, maps: &Vec<Recipe>) -> u64 {
    let mut result: u64 = n;
    for m in maps {
        result = m.map(result);
    }
    result
}

fn parse(s: &str) -> (Vec<u64>, Vec<(u64, u64)>, Vec<Recipe>) {
    let parts: Vec<_> = s.split("\n\n").collect();

    let seeds: Vec<u64> = parts.get(0).unwrap()[7..]
        .split(" ")
        .map(|s| s.parse().unwrap())
        .collect();

    let ranges: Vec<(u64, u64)> = seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();

    let mut recipes: Vec<Recipe> = vec![];
    for p in parts.iter().skip(1) {
        let (_, numbers) = p.split(" map:\n").collect_tuple().unwrap();

        let mut maps: Vec<(u64, u64, u64)> = vec![];

        for ns in numbers.lines() {
            let (d, s, l) = ns
                .split(" ")
                .map(|s| s.parse().unwrap())
                .collect_tuple()
                .unwrap();
            maps.push((d, s, l));
        }

        recipes.push(Recipe {
            maps: maps,
        });
    }

    (seeds, ranges, recipes)
}

fn part1(seeds: &Vec<u64>, rs: &Vec<Recipe>) -> u64 {
    seeds.iter().map(|i| apply_recipes(*i, rs)).min().unwrap()
}

fn part2(ranges: &Vec<(u64, u64)>, rs: &Vec<Recipe>) -> u64 {
    let mut min: u64 = u64::MAX;

    for (b, l) in ranges {
        print!("Range {}..{} -> new min: ", b, b+l);

        for i in *b..(b + l) {
            min = u64::min(min, apply_recipes(i, rs));
        }

        println!("{min}");
    }

    min
}

fn main() {
    let file_path = "./day05/input.txt";
    let content = fs::read_to_string(file_path).unwrap();
    let (seeds, ranges, rs) = parse(&content);

    println!("Result part1: {}", part1(&seeds, &rs));
    println!("Result part2: {}", part2(&ranges, &rs));
}
