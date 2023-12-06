#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn main() {
    let input = include_str!("./input6.txt");
    let out = part1(input);
    println!("{out}");
}

fn parse_input(input: &str) -> Vec<Race> {
    let lines: Vec<&str> = input.lines().collect();

    let time_inputs: Vec<u32> = lines[0]
        .replace("Time:", "")
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    let distance_inputs: Vec<u32> = lines[1]
        .replace("Distance:", "")
        .trim()
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    time_inputs
        .iter()
        .zip(distance_inputs.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect()
}

fn calculate_max(time: u32, distance: u32) -> u32 {
    let mut charge_time = time;
    let mut remaining_time = 0;

    while distance >= remaining_time * (charge_time) {
        charge_time -= 1;
        remaining_time = time - charge_time;
    }

    charge_time
}

fn calculate_min(time: u32, distance: u32) -> u32 {
    let mut charge_time = 1;
    let mut remaining_time = time - charge_time;

    while distance >= remaining_time * (charge_time) {
        charge_time += 1;
        remaining_time = time - charge_time;
    }

    charge_time
}

fn part1(input: &str) -> String {
    let data: Vec<Race> = parse_input(input);

    let res: u32 = data
        .iter()
        .map(|race| {
            calculate_max(race.time, race.distance) - calculate_min(race.time, race.distance) + 1
        })
        .product();

    res.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn it_works() {
        let result = part1(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "288");
    }
}
