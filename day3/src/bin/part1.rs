struct DigitSequence {
    value: usize,
    x_start: usize,
    x_end: usize,
    y: usize,
}

const NOT_PART_NUMBER_REGEX: &str = r"[^.0-9]";

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
            if is_part_number(&digit_sequence, grid) {
                part_numbers.push(digit_sequence);
            }
        }
    }
    part_numbers
}

fn get_digits_from_line(line: Vec<&str>, y: usize) -> Vec<DigitSequence> {
    let mut digit_sequences = Vec::new();
    let (mut x_start, mut x_end) = (0, 0);
    for (x, character) in line.iter().enumerate() {
        if !character.matches(NOT_PART_NUMBER_REGEX).collect::<Vec<&str>>().is_empty() {
            if x_start != x_end {
                digit_sequences.push(DigitSequence {
                    value: line[x_start..x_end].join("").parse::<usize>().unwrap_or(0),
                    x_start,
                    x_end,
                    y,
                });
            }
            x_start = x + 1;
            x_end = x + 1;
        } else {
            x_end += 1;
        }
    }
    digit_sequences
}

fn is_part_number(sequence: &DigitSequence, grid: &Vec<Vec<&str>>) -> bool {
    let mut is_part_number = true;
    for y in sequence.y - 1..=sequence.y + 1 {
        for x in sequence.x_start..sequence.x_end {
            if !grid[y][x].matches(NOT_PART_NUMBER_REGEX).collect::<Vec<&str>>().is_empty() {
                is_part_number = false;
            }
        }
    }
    is_part_number
}
