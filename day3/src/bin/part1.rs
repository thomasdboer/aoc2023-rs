fn main() {
    let raw_input = std::fs::read_to_string("input.txt").unwrap();
    let input = parse_input(&raw_input);
}

fn parse_input(input: &str) -> Vec<Vec<&str>> {
    input
        .split("\n")
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}
