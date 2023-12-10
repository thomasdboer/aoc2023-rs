struct ValueMap {
    source_range_start: u32,
    destination_range_start: u32,
    range_length: u32,
}

struct Seed {
    value: u32,
}

struct Puzzle {
    value_maps: Vec<Vec<ValueMap>>,
    seeds: Vec<Seed>,
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let sections = input.split(":").collect::<Vec<&str>>();
    let puzzle: Puzzle = parse_input(sections);
    let location_numbers = puzzle.seeds
        .iter()
        .map(|seed| {
            println!("Seed: {}", seed.value);
            find_location(seed, 0, &puzzle)
        })
        .collect::<Vec<u32>>();
    println!("Answer: {:?}", location_numbers.iter().min().unwrap());
}

fn find_location(seed: &Seed, round: usize, puzzle: &Puzzle) -> u32 {
    let mut new_seed = seed.value;
    let old_seed = new_seed.clone();
    for map in &puzzle.value_maps[round] {
        if
            old_seed >= map.source_range_start &&
            old_seed < map.source_range_start + map.range_length
        {
            new_seed = map.destination_range_start + (old_seed - map.source_range_start);
            break;
        }
    }

    if round < puzzle.value_maps.len() - 1 {
        new_seed = find_location(&(Seed { value: new_seed }), round + 1, puzzle);
    }
    new_seed
}

fn parse_input(sections: Vec<&str>) -> Puzzle {
    Puzzle {
        seeds: sections[1]
            .split("\n")
            .take(1)
            .flat_map(|line| line.split(" "))
            .filter(|value| value.chars().all(|c| c.is_numeric()) && !value.is_empty())
            .map(|value| Seed { value: value.parse::<u32>().unwrap() })
            .collect::<Vec<Seed>>(),

        value_maps: sections
            .iter()
            .skip(1)
            .map(|section| { value_maps_from_section(section) })
            .collect::<Vec<Vec<ValueMap>>>(),
    }
}

fn value_maps_from_section(section: &str) -> Vec<ValueMap> {
    section
        .split("\n")
        .filter(|line| !line.contains("map") && !line.is_empty())
        .map(|line| value_map_from_line(&line))
        .collect::<Vec<ValueMap>>()
}

fn value_map_from_line(line: &str) -> ValueMap {
    let values = line
        .split(" ")
        .filter(|value| { value.chars().all(|c| c.is_numeric()) && !value.is_empty() })
        .map(|value| value.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    ValueMap {
        destination_range_start: values[0],
        source_range_start: values[1],
        range_length: values[2],
    }
}
