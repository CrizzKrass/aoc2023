#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn main() {
    let input = include_str!("./input6.txt");
    let out = part2(input);
    println!("{out}");
}

fn parse_input(input: &str) -> Race {
    let lines: Vec<&str> = input.lines().collect();

    let time: u64 = lines[0]
        .replace("Time:", "")
        .replace(" ", "")
        .trim()
        .parse()
        .unwrap();
        

    let distance: u64 = lines[1]
        .replace("Distance:", "")
        .replace(" ", "")
        .trim()
        .parse()
        .unwrap();
        
    Race {time, distance}
}

fn calculate_max(time: u64, distance: u64) -> u64 {
    let mut charge_time = time;
    let mut remaining_time = 0;

    while distance >= remaining_time * (charge_time) {
        charge_time -= 1;
        remaining_time = time - charge_time;
    }

    charge_time
}

fn calculate_min(time: u64, distance: u64) -> u64 {
    let mut charge_time = 1;
    let mut remaining_time = time - charge_time;

    while distance >= remaining_time * (charge_time) {
        charge_time += 1;
        remaining_time = time - charge_time;
    }

    charge_time
}

fn part2(input: &str) -> String {
    let data: Race = parse_input(input);

    (calculate_max(data.time, data.distance) - calculate_min(data.time, data.distance) + 1).to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;
    #[test]
    fn it_works() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "71503");
    }
}
