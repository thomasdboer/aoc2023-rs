// Example games:
// Game 1: 4 red, 5 blue, 9 green; 7 green, 7 blue, 3 red; 16 red, 7 blue, 3 green; 11 green, 11 blue, 6 red; 12 red, 14 blue
// Game 2: 12 blue, 11 green, 3 red; 6 blue, 5 green, 7 red; 5 red, 11 blue; 2 blue, 8 green
// Game 3: 8 blue, 5 green, 2 red; 5 blue, 5 green, 7 red; 7 blue, 1 green, 7 red; 8 green, 14 blue, 7 red; 8 green, 14 blue; 8 blue, 2 green, 8 red
// Game 4: 3 red, 14 blue, 15 green; 1 red, 11 green, 14 blue; 14 green, 17 blue
// Game 5: 11 green, 2 red, 10 blue; 16 green, 8 blue; 2 blue, 6 green, 1 red; 14 blue, 2 red, 16 green; 3 blue, 18 green; 1 red, 10 blue, 3 green
// Game 6: 2 green, 5 red, 17 blue; 12 green, 13 blue, 6 red; 8 red, 9 blue, 7 green
// Game 7: 2 blue, 18 green; 4 green, 1 red, 1 blue; 4 blue, 19 green
// Game 8: 6 green, 7 blue; 9 green, 6 blue; 7 blue, 1 red, 3 green

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
    let mut possible_games = get_possible_games(games);
    let answer: usize = possible_games
        .iter()
        .map(|game| game.id)
        .collect::<Vec<usize>>()
        .iter()
        .sum();
    println!("Answer: {}", answer);
}

fn get_possible_games(games: Vec<Game>) -> Vec<Game> {
    const MAX_HAND: Hand = Hand { blue: 12, green: 13, red: 14 };
    let mut possible_games = Vec::new();
    for game in games {
        let mut possible = true;
        for hand in &game.hands {
            if hand.blue > MAX_HAND.blue || hand.green > MAX_HAND.green || hand.red > MAX_HAND.red {
                possible = false;
                break;
            }
        }
        if possible {
            possible_games.push(game);
        }
    }
    possible_games
}

fn parse_input(input: &str) -> Vec<Game> {
    let mut games = Vec::new();
    for line in input.lines() {
        let mut game_with_identifier = line.split(": ");
        let mut game_id = game_with_identifier.next().unwrap().split(" ");
        let id = match game_id.next().unwrap().parse::<usize>() {
            Err(_) => panic!("Invalid game id: {}", game_id.next().unwrap()),
            Ok(id) => id,
        };
        println!("Game id: {}", id);
        let mut game = Game {
            id: id,
            hands: Vec::new(),
        };
        for raw_hand in line.split("; ") {
            let mut hand = Hand {
                blue: 0,
                green: 0,
                red: 0,
            };
            for cube in raw_hand.split(", ") {
                let mut cube2 = cube.split(" ");
                let count = match cube2.next().unwrap().parse::<usize>() {
                    Err(_) => panic!("Invalid count: {}", cube2.next().unwrap()),
                    Ok(count) => count,
                };
                let color = cube2.next().unwrap();
                match color {
                    "blue" => {
                        hand.blue = count;
                    }
                    "green" => {
                        hand.green = count;
                    }
                    "red" => {
                        hand.red = count;
                    }
                    _ => panic!("Unknown color: {}", color),
                }
            }
            game.hands.push(hand);
        }
        games.push(game);
    }
    games
}
