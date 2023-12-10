use std::collections::HashMap;

#[derive(Debug)]
struct Possibilities {
    L: String,
    R: String,
}

fn main() {
    let input = include_str!("./input8.txt");
    let out = part2(input);
    println!("{out}");
}

fn parse_possibilities(lines: Vec<&str>) -> HashMap<&str, Possibilities> {
    let pos = lines.iter().fold(HashMap::new(), |mut acc, line| {
        let split_line: Vec<&str> = line.split("=").collect();
        let key = split_line[0].trim();
        let pos: Vec<String> = split_line[1]
            .trim()
            .split(",")
            .map(|p| {
                p.chars()
                    .filter(|x| x.is_ascii_alphanumeric())
                    .collect::<String>()
            })
            .collect();

        acc.insert(
            key,
            Possibilities {
                L: pos[0].clone(),
                R: pos[1].clone(),
            },
        );
        acc
    });
    pos
}

fn find_lcm(a: usize, b:usize) -> usize {
    let greater = a.max(b);
    let smaller = a.min(b);

    let mut i = greater;
    while i % smaller != 0 {
        i += greater;
    }

    i
}

fn part2(input: &str) -> String {
    let input = input
        .lines()
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    let instructions: Vec<char> = input[0].chars().collect();

    let possibilities = parse_possibilities(input[1..].to_vec());

    let repeat = instructions.len();
    let starting_positions: Vec<&str> = possibilities
        .keys()
        .filter(|x| x.chars().last().unwrap() == 'A')
        .map(|key| *key)
        .collect::<Vec<&str>>();

    let mut counts: Vec<usize> = Vec::new();

    for pos in starting_positions {
        let mut cur_pos = pos;
        let mut index = 0;

        while cur_pos.chars().last().unwrap() != 'Z' {
            if instructions[index % repeat] == 'L' {
                cur_pos = &possibilities.get(cur_pos).unwrap().L;
            } else if instructions[index % repeat] == 'R' {
                cur_pos = &possibilities.get(cur_pos).unwrap().R;
            }

            index += 1;
        }

        counts.push(index)
    }

    counts.into_iter().reduce(|acc, count| find_lcm(acc, count)).unwrap().clone().to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;
    #[test]
    fn it_works() {
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX",
        );
        assert_eq!(result, "6");
    }
}
