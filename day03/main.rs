use regex_lite::Regex;
use std::fs;

#[derive(Debug)]
struct Number {
    num: u32,
    row: i32,
    col_start: i32,
    col_end: i32,
}

#[derive(Debug)]
struct Symbol {
    character: char,
    row: i32,
    col: i32,
}

fn find_numbers(map: &str) -> Vec<Number> {
    let re = Regex::new(r"\d+").unwrap();

    let mut result = vec![];
    for (i, line) in map.lines().enumerate() {
        for m in re.find_iter(line) {
            result.push(Number {
                num: m.as_str().parse().unwrap(),
                row: i as i32,
                col_start: m.start() as i32,
                col_end: m.end() as i32 - 1,
            });
        }
    }

    result
}

fn find_symbols(map: &str) -> Vec<Symbol> {
    let re = Regex::new(r"[^\.\d]").unwrap();

    let mut result = vec![];
    for (i, line) in map.lines().enumerate() {
        for m in re.find_iter(line) {
            result.push(Symbol {
                character: m.as_str().parse().unwrap(),
                row: i as i32,
                col: m.start() as i32,
            });
        }
    }

    result
}

fn neighbors(number: &Number, symbol: &Symbol) -> bool {
    if i32::abs(number.row - symbol.row) > 1 {
        return false;
    }

    if i32::abs(number.col_start - symbol.col) <= 1 {
        return true;
    }

    if i32::abs(number.col_end - symbol.col) <= 1 {
        return true;
    }

    false
}

fn part1(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> u32 {
    let mut result = 0;

    for n in numbers {
        for s in symbols {
            if neighbors(n, s) {
                result += n.num;
            }
        }
    }

    result
}

fn part2(numbers: &Vec<Number>, symbols: &Vec<Symbol>) -> u32 {
    let mut result = 0;

    for s in symbols {
        if s.character != '*' {
            continue;
        };

        let neighbors: Vec<_> = numbers.iter().filter(|n| neighbors(n, s)).collect();

        if neighbors.len() != 2 {
            continue;
        }

        result += neighbors[0].num * neighbors[1].num;
    }

    result
}

fn main() {
    let file_path = "./day03/input.txt";
    let content = fs::read_to_string(file_path).unwrap();

    let numbers = find_numbers(&content);
    let symbols = find_symbols(&content);

    println!("Result part1: {}", part1(&numbers, &symbols));
    println!("Result part2: {}", part2(&numbers, &symbols));
}
