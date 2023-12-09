struct ValueMap {
    source_range_start: usize,
    destination_range_start: usize,
    range_length: usize,
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
}

// example input
// seeds: 79 14 55 13

// seed-to-soil map:
// 50 98 2
// 52 50 48

// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15

// fertilizer-to-water map:
// 49 53 8
// 0 11 42
// 42 0 7
// 57 7 4

// water-to-light map:
// 88 18 7
// 18 25 70

// light-to-temperature map:
// 45 77 23
// 81 45 19
// 68 64 13

// temperature-to-humidity map:
// 0 69 1
// 1 0 69

// humidity-to-location map:
// 60 56 37
// 56 93 4
fn parse_input(input: &str) -> Vec<Vec<ValueMap>> {
    let mut result = Vec::new();
    let parts = input.split(":").collect::<Vec<&str>>();
    let parts_iter = parts.iter();
    let seeds = &parts_iter
        .next()
        .unwrap()
        .split(" ")
        .map(|seed| seed.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    // For every line in the seed to soil map, construct a ValueMap of each of the three values in the line.
    let seed_to_soil_map = &parts_iter
        .next()
        .unwrap()
        .split("\n")
        .map(|line| {
            let x = line.split(" ").next();
            println!("X: {:?}", x);
            return ValueMap {
                source_range_start: 0,
                destination_range_start: 0,
                range_length: 0,
            };
        })
        .collect::<Vec<ValueMap>>();
    result.push(seed_to_soil_map);
    result
}
