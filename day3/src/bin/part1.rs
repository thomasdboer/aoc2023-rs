use regex::Regex;

struct DigitSequence {
    value: usize,
    x_start: usize,
    x_end: usize,
    y: usize,
}

fn main() {
    let raw_input = std::fs::read_to_string("input.txt").unwrap();
    let grid = parse_input(&raw_input);
    let part_numbers = find_part_numbers(&grid);
    let answer: usize = part_numbers
        .iter()
        .map(|part_number| part_number.value)
        .sum();
    println!("Answer: {}", answer);
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n")
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}

fn find_part_numbers(grid: &Vec<Vec<&str>>) -> Vec<DigitSequence> {
    let mut part_numbers: Vec<DigitSequence> = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        let digit_sequences: Vec<DigitSequence> = get_digits_from_line(line.clone(), y);
        for digit_sequence in digit_sequences {
            println!("{}", digit_sequence.value);
            if is_part_number(&digit_sequence, grid) {
                part_numbers.push(digit_sequence);
            }
        }
    }
    part_numbers
}

fn get_digits_from_line(line: Vec<&str>, y: usize) -> Vec<DigitSequence> {
    let number_regex: Regex = Regex::new(r"\d").unwrap();
    let mut digit_sequences = Vec::new();
    let (mut x_start, mut x_end) = (0, 0);
    for (x, character) in line.iter().enumerate() {
        if !number_regex.is_match(character) {
            if x_start != x_end {
                digit_sequences.push(DigitSequence {
                    value: line[x_start..x_end].join("").parse::<usize>().unwrap_or(0),
                    x_start,
                    x_end,
                    y,
                });
                digit_sequences
                    .iter()
                    .for_each(|digit_sequence| { println!("{}", digit_sequence.value) });

                x_start = x + 1;
                x_end = x + 1;
            } else {
                x_end += 1;
            }
        }
    }
    digit_sequences
}

fn is_part_number(sequence: &DigitSequence, grid: &Vec<Vec<&str>>) -> bool {
    let part_number_regex: Regex = Regex::new(r"[^\d.]").unwrap();
    let mut is_part_number = false;
    let (x_start, x_end, y) = (sequence.x_start, sequence.x_end, sequence.y);
    let mut adjacent_characters = Vec::new();
    if y > 0 {
        adjacent_characters.push(grid[y - 1][x_start - 1..x_end + 1].join(""));
    }
    adjacent_characters.push(grid[y][x_start - 1..x_end + 1].join(""));
    if y < grid.len() - 1 {
        adjacent_characters.push(grid[y + 1][x_start - 1..x_end - 1].join(""));
    }
    for adjacent_character in adjacent_characters {
        if !part_number_regex.is_match(&adjacent_character) {
            is_part_number = true;
        }
    }
    println!("{}", is_part_number.to_string());
    is_part_number
}
