#[derive(Clone)]
struct Card {
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

fn main() {
    let raw_input = std::fs::read_to_string("input.txt").unwrap();
    let cards: Vec<Card> = parse_input(&raw_input);
    let answer = count_resulting_cards(&cards);
    println!("Answer: {}", answer);
}

fn count_resulting_cards(cards: &Vec<Card>) -> u64 {
    let mut copies = vec![1u64; cards.len()];

    for (i, Card { winning_numbers, my_numbers, .. }) in cards.iter().enumerate() {
        let number_of_wins = winning_numbers
            .iter()
            .filter(|number| my_numbers.contains(number))
            .count();
        if number_of_wins == 0 {
            continue;
        }
        let repeats = copies[i];
        for copy in &mut copies[i + 1..=i + number_of_wins] {
            *copy += repeats;
        }
    }
    copies.iter().sum()
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .trim()
        .split("\n")
        .map(|line| get_card(line))
        .collect::<Vec<Card>>()
}

fn get_card(line: &str) -> Card {
    let parts = line.split(":").collect::<Vec<&str>>();
    let mut numbers = parts[1].split("|");
    let winning_numbers = numbers
        .next()
        .unwrap()
        .split(" ")
        .filter(|number| !number.is_empty())
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let my_numbers = numbers
        .next()
        .unwrap()
        .split(" ")
        .filter(|number| !number.is_empty())
        .map(|number| number.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    Card {
        my_numbers,
        winning_numbers,
    }
}
