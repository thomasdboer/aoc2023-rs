struct Game {
    hands: Vec<Hand>,
}

struct Hand {
    blue: usize,
    green: usize,
    red: usize,
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let games = parse_input(&input);
    let answer = games
        .iter()
        .map(|game| { get_game_score(game) })
        .sum::<usize>();
    println!("Answer: {}", answer);
}

fn get_game_score(game: &Game) -> usize {
    let mut blue = 0;
    let mut green = 0;
    let mut red = 0;
    for hand in &game.hands {
        if hand.blue > blue {
            blue = hand.blue;
        }
        if hand.green > green {
            green = hand.green;
        }
        if hand.red > red {
            red = hand.red;
        }
    }
    blue * green * red
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .split("\n")
        .map(|line| parse_game(line))
        .collect::<Vec<Game>>()
}

fn parse_game(line: &str) -> Game {
    let mut hands = Vec::new();
    for (j, part) in line.split(": ").enumerate() {
        if j > 0 {
            hands = part
                .split("; ")
                .map(|hand| parse_hand(hand))
                .collect::<Vec<Hand>>();
        }
    }
    Game { hands }
}

fn parse_hand(hand: &str) -> Hand {
    let mut blue = 0;
    let mut green = 0;
    let mut red = 0;
    for part in hand.split(", ") {
        let mut parts = part.split(" ");
        let count = parts.next().unwrap().parse::<usize>().unwrap();
        let color = parts.next().unwrap();
        match color {
            "blue" => {
                blue = count;
            }
            "green" => {
                green = count;
            }
            "red" => {
                red = count;
            }
            _ => panic!("Unknown color: {}", color),
        }
    }
    Hand { blue, green, red }
}
