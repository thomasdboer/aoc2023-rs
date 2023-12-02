const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

struct Game {
    id: usize,
    hands: Vec<Hand>,
}

struct Hand {
    blue: usize,
    green: usize,
    red: usize,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let games = parse_input(&input);
    let possible_games = get_possible_games(games);
    let answer: usize = possible_games
        .iter()
        .map(|game| game.id)
        .collect::<Vec<usize>>()
        .iter()
        .sum();
    println!("Answer: {}", answer);
}

fn get_possible_games(games: Vec<Game>) -> Vec<Game> {
    games
        .into_iter()
        .filter(|game| {
            game.hands
                .iter()
                .all(|hand| {
                    hand.blue <= MAX_BLUE && hand.green <= MAX_GREEN && hand.red <= MAX_RED
                })
        })
        .collect::<Vec<Game>>()
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .split("\n")
        .map(|line| parse_game(line))
        .collect::<Vec<Game>>()
}

fn parse_game(line: &str) -> Game {
    let mut hands = Vec::new();
    let mut id = 0;
    for (j, part) in line.split(": ").enumerate() {
        if j == 0 {
            id = part.split(" ").last().unwrap_or("0").parse::<usize>().unwrap_or(0);
        } else {
            hands = part
                .split("; ")
                .map(|hand| parse_hand(hand))
                .collect::<Vec<Hand>>();
        }
    }
    Game { id, hands }
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
