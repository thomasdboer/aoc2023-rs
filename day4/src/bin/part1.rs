struct Card {
    winning_numbers: Vec<usize>,
    my_numbers: Vec<usize>,
}

const SCORE_FACTOR: usize = 2;

fn main() {
    let raw_input = std::fs::read_to_string("input.txt").unwrap();
    let input: Vec<Card> = parse_input(&raw_input);
    let answer = input
        .iter()
        .map(|card| calculate_card_score(card))
        .sum::<usize>();
    println!("Answer: {}", answer);
}
fn calculate_card_score(card: &Card) -> usize {
    let mut score = 0;
    for my_number in &card.my_numbers {
        if card.winning_numbers.contains(my_number) {
            if score == 0 {
                score = 1;
            } else {
                score *= SCORE_FACTOR;
            }
        }
    }
    score
}
fn parse_input(input: &str) -> Vec<Card> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            println!("Line: {:?}", line);
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
        })
        .collect::<Vec<Card>>()
}
