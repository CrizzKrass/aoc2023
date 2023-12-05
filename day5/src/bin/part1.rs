#[derive(Debug)]
struct GardeningMap {
    src_start: u128,
    dst_start: u128,
    range: u128,
}

fn main() {
    let input = include_str!("./input5.txt");
    let out = part1(input);
    println!("{out}");
}

fn get_associated_value(map: &Vec<GardeningMap>, src: u128) -> u128 {
    match map.iter().filter(|item| item.src_start <= src && item.src_start+item.range > src).take(1).next() {
        Some(item) => {
            let diff = src - item.src_start;
            item.dst_start + diff
        },
        None => src,
    }
}

fn part1(input: &str) -> String {
    //let paragraphs: Vec<_> = input.split("\n\n").filter(|x| !x.is_empty()).collect();
    let paragraphs: Vec<_> = input.split("\r\n\r\n").filter(|x| !x.is_empty()).collect();
    
    let seeds: Vec<u128> = paragraphs[0]
        .replace("seeds:", "")
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let seed_to_soil_map: Vec<GardeningMap> = paragraphs[1]
        .lines()
        .skip(1)
        .map(|line| {
            let line: Vec<u128> = line
                .trim()
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            GardeningMap {
                dst_start: line[0],
                src_start: line[1],
                range: line[2],
            }
        })
        .collect();

    let soil_to_fertilizer_map: Vec<GardeningMap> = paragraphs[2]
        .lines()
        .skip(1)
        .map(|line| {
            let line: Vec<u128> = line
                .trim()
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            GardeningMap {
                dst_start: line[0],
                src_start: line[1],
                range: line[2],
            }
        })
        .collect();

    let fertilizer_to_water_map: Vec<GardeningMap> = paragraphs[3]
        .lines()
        .skip(1)
        .map(|line| {
            let line: Vec<u128> = line
                .trim()
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            GardeningMap {
                dst_start: line[0],
                src_start: line[1],
                range: line[2],
            }
        })
        .collect();

    let water_to_light_map: Vec<GardeningMap> = paragraphs[4]
        .lines()
        .skip(1)
        .map(|line| {
            let line: Vec<u128> = line
                .trim()
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            GardeningMap {
                dst_start: line[0],
                src_start: line[1],
                range: line[2],
            }
        })
        .collect();

    let light_to_temperature_map: Vec<GardeningMap> = paragraphs[5]
        .lines()
        .skip(1)
        .map(|line| {
            let line: Vec<u128> = line
                .trim()
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            GardeningMap {
                dst_start: line[0],
                src_start: line[1],
                range: line[2],
            }
        })
        .collect();

    let temperature_to_humidity_map: Vec<GardeningMap> = paragraphs[6]
        .lines()
        .skip(1)
        .map(|line| {
            let line: Vec<u128> = line
                .trim()
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            GardeningMap {
                dst_start: line[0],
                src_start: line[1],
                range: line[2],
            }
        })
        .collect();

    let humidity_to_location_map: Vec<GardeningMap> = paragraphs[7]
        .lines()
        .skip(1)
        .map(|line| {
            let line: Vec<u128> = line
                .trim()
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            GardeningMap {
                dst_start: line[0],
                src_start: line[1],
                range: line[2],
            }
        })
        .collect();

    let mut res: Vec<u128> = Vec::new();

    for seed in seeds {
        let soil = get_associated_value(&seed_to_soil_map, seed);
        let fertilizer = get_associated_value(&soil_to_fertilizer_map, soil);
        let water = get_associated_value(&fertilizer_to_water_map, fertilizer);
        let light = get_associated_value(&water_to_light_map, water);
        let temperature = get_associated_value(&light_to_temperature_map, light);
        let humidity = get_associated_value(&temperature_to_humidity_map, temperature);
        let location = get_associated_value(&humidity_to_location_map, humidity);
        res.push(location);
    }

    res.iter().min().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn it_works() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, "35");
    }
}
