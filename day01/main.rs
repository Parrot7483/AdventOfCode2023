use regex_lite::Regex;
use std::fs;

fn num_to_int(num: &str) -> u32 {
    if let Ok(i) = num.parse::<u32>() {
        return i
    }

    match num {
        "one" => return 1,
        "two" => return 2,
        "three" => return 3,
        "four" => return 4,
        "five" => return 5,
        "six" => return 6,
        "seven" => return 7,
        "eight" => return 8,
        "nine" => return 9,
        _ => panic!("Can not convert string: {num}")
    }
}

fn nums(line: &str) -> (u32, u32) {
    let numbers = r"one|two|three|four|five|six|seven|eight|nine|[1-9]";
    let double = format!(r"({numbers}).*({numbers})");
    let single = format!(r"({numbers})");
    let combined = format!(r"(?:{double})|(?:{single})");
    let re = Regex::new(&combined).unwrap();

    if let Some(captures) = re.captures(line) {
        match (captures.get(1), captures.get(2), captures.get(3)) {
            (Some(a), Some(b), None) => {
                let num1 = num_to_int(a.as_str());
                let num2 = num_to_int(b.as_str());
                return (num1, num2)
            }
            (None, None, Some(a)) => {
                let num1 = num_to_int(a.as_str());
                return (num1, num1)
            }
            _ => panic!("No numbers found in captures: {captures:?}");
        }
    }

    panic!("No captures found: {line}");
}

fn main() {
    let file_path = "./day01/input.txt";
    let input = fs::read_to_string(file_path).unwrap();

    let mut result: u32 = 0;

    for line in input.lines() {
        let (a, b) = nums(line);
        result += a * 10 + b;
    }

    println!("Result: {result}")
}
