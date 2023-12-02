use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n");
    let sum: i32 = lines.map(|line| line_value(line)).sum();
    println!("{}", sum)
}

fn line_value(line: &str) -> i32 {
    let first_digit = greedy_find_digit(line, false);
    let second_digit = greedy_find_digit(line, true);
    let result = format!("{}{}", first_digit, second_digit)
        .parse::<i32>()
        .unwrap();
    println!("{} {}", line, result);
    result
}

fn greedy_find_digit(line: &str, reverse: bool) -> i32 {
    let target_words: HashMap<String, i32> = match reverse {
        true => number_map()
            .iter()
            .map(|(key, &value)| (reverse_slice(&key), value))
            .collect(),
        false => number_map(),
    };
    let search_string: String = match reverse {
        true => reverse_slice(line),
        false => String::from(line)
    };
    let max_window_size = target_words
        .iter()
        .map(|(i, _j)| i.chars().count())
        .max()
        .unwrap();
    
    for (i, c) in search_string.chars().enumerate() {
        if c.is_digit(10) {
            return search_string[i..i + 1].parse::<i32>().unwrap();
        }
        let window_start = match i.overflowing_sub(max_window_size) {
            (x, false) => x,
            (_, true) => 0
        };
        for j in window_start..i {
            let window = &search_string[j..i];
            if target_words.contains_key(window) {
                return target_words[window]
            }
        }
    }
    0
}

fn reverse_slice(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn number_map() -> HashMap<String, i32> {
    return HashMap::from([(String::from("one"), 1),
    (String::from("two"), 2), 
    (String::from("three"), 3), 
    (String::from("four"), 4), 
    (String::from("five"), 5), 
    (String::from("six"), 6), 
    (String::from("seven"), 7), 
    (String::from("eight"), 8), 
    (String::from("nine"), 9), 
    (String::from("ten"), 10)]);
}