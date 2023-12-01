use std::fs::{self};

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n");
    let mut count: i32 = 0;
    for line in lines {
        count += line_value(line);
    }
    println!("{}", count);
}

fn line_value(line: &str) -> i32 {
    let first_digit = get_digit(line);
    let second_digit = get_digit(&reverse(line));
    let result = format!("{}{}", first_digit, second_digit)
        .parse::<i32>()
        .unwrap();
    result
}

fn get_digit(line: &str) -> i32 {
    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            return line[i..i + 1].parse::<i32>().unwrap();
        }
    }
    0
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect::<String>()
}
