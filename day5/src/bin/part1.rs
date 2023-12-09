struct ValueMap {
    source_range_start: usize,
    destination_range_start: usize,
    range_length: usize,
}

struct Seed {
    value: usize,
}

struct Puzzle {
    value_maps: Vec<Vec<ValueMap>>,
    seeds: Vec<Seed>,
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();
    let puzzle: Puzzle = parse_input(&input);
    let location_numbers = puzzle.seeds
        .iter()
        .map(|seed| find_location(seed, 0, &puzzle))
        .collect::<Vec<usize>>();
    println!("Answer: {:?}", location_numbers.iter().min());
}

fn find_location(seed: &Seed, round: usize, puzzle: &Puzzle) -> usize {
    let mut new_seed = seed.value;
    for map in &puzzle.value_maps[round] {
        if
            new_seed >= map.source_range_start &&
            new_seed < map.source_range_start + map.range_length
        {
            new_seed = map.destination_range_start + (new_seed - map.source_range_start);
        }
    }
    if round < puzzle.value_maps.len() - 1 {
        new_seed = find_location(&(Seed { value: new_seed }), round + 1, puzzle);
    }
    new_seed
}

fn parse_input(input: &str) -> Puzzle {
    let sections: Vec<&str> = input.split(":").collect::<Vec<&str>>();

    // the second section after the colon contains the seeds
    let seeds: Vec<Seed> = sections[1]
        .split(" ")
        .filter(|value| { value.chars().all(|c| c.is_numeric()) && !value.is_empty() })
        .map(|value| Seed { value: value.parse::<usize>().unwrap() })
        .collect::<Vec<Seed>>();

    let value_maps: Vec<Vec<ValueMap>> = sections
        .iter()
        .skip(1)
        .map(|section| {
            section
                .split("\n")
                .filter(|line| !line.contains("map") && !line.is_empty())
                .map(|line| value_map_from_line(&line))
                .collect::<Vec<ValueMap>>()
        })
        .collect::<Vec<Vec<ValueMap>>>();

    Puzzle {
        value_maps,
        seeds,
    }
}

fn value_map_from_line(line: &str) -> ValueMap {
    let values = line
        .split(" ")
        .filter(|value| { value.chars().all(|c| c.is_numeric()) && !value.is_empty() })
        .map(|value| value.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    ValueMap {
        source_range_start: values[0],
        destination_range_start: values[1],
        range_length: values[2],
    }
}
