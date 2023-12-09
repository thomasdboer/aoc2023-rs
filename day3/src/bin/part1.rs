use regex::Regex;

struct DigitSequence {
    value: usize,
    x_start: usize,
    x_end: usize,
    y: usize,
}

fn main() {
    let raw_input = std::fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<&str>> = parse_input(&raw_input);
    let part_numbers: Vec<DigitSequence> = find_part_numbers(&grid);
    for part_number in &part_numbers {
        print!("{} ", part_number.value);
    }
    let answer: usize = part_numbers
        .iter()
        .map(|part_number: &DigitSequence| part_number.value)
        .sum();
    println!("Answer: {}", answer);
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .trim_end()
        .split("\n")
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}

fn find_part_numbers(grid: &Vec<Vec<&str>>) -> Vec<DigitSequence> {
    let mut part_numbers: Vec<DigitSequence> = Vec::new();
    for (y, line) in grid.iter().enumerate() {
        let digit_sequences: Vec<DigitSequence> = get_digits_from_line(line.to_vec(), y);
        for digit_sequence in digit_sequences {
            if is_part_number(&digit_sequence, grid) {
                part_numbers.push(digit_sequence);
            }
        }
    }
    println!("Part numbers: {:?}", part_numbers.len());
    println!("Grid: {:?}", grid.len());
    part_numbers
}

fn get_digits_from_line(line: Vec<&str>, y: usize) -> Vec<DigitSequence> {
    let number_regex = Regex::new(r"[0-9]").unwrap();
    let mut digit_sequences = Vec::new();
    let (mut x_start, mut x_end) = (0, 0);
    for (x, character) in line.iter().enumerate() {
        if !number_regex.is_match(character) {
            if x_end > x_start {
                digit_sequences.push(DigitSequence {
                    value: line[x_start..x_end].join("").parse::<usize>().unwrap_or(0),
                    x_start,
                    x_end,
                    y,
                });
            }
            x_start = x + 1;
        }
        x_end += 1;
    }
    digit_sequences
}

fn is_part_number(sequence: &DigitSequence, grid: &Vec<Vec<&str>>) -> bool {
    let not_part_number_regex = Regex::new(r"[0-9.]").unwrap();
    let surrounding_characters: Vec<&str> = get_surrounding_characters(sequence, grid);
    !surrounding_characters
        .iter()
        .filter(|x| { *x != &"" })
        .all(|character| not_part_number_regex.is_match(character))
}

fn get_surrounding_characters<'a>(
    sequence: &DigitSequence,
    grid: &'a Vec<Vec<&str>>
) -> Vec<&'a str> {
    let (x_max, y_max) = (grid[0].len() - 1, grid.len() - 1);
    let (x_min, y_min) = (0, 0);
    let (y1, y2, x1, x2) = (
        if sequence.y > y_min { sequence.y - 1 } else { sequence.y },
        if sequence.y < y_max { sequence.y + 1 } else { sequence.y },
        if sequence.x_start > x_min { sequence.x_start - 1 } else { sequence.x_start },
        if sequence.x_end < x_max { sequence.x_end + 1 } else { sequence.x_end },
    );
    grid[y1..=y2]
        .iter()
        .flat_map(|line| { line[x1..=x2].to_vec() })
        .collect::<Vec<&'a str>>()
}
