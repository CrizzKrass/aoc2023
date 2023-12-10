use std::collections::HashMap;

#[derive(Debug)]
struct Possibilities {
    L: String,
    R: String,
}

fn main() {
    let input = include_str!("./input8.txt");
    let out = part1(input);
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
                    .filter(|x| x.is_ascii_alphabetic())
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

fn part1(input: &str) -> String {
    let input = input
        .lines()
        .filter(|x| !x.is_empty())
        .collect::<Vec<&str>>();
    let instructions: Vec<char> = input[0].chars().collect();

    let possibilities = parse_possibilities(input[1..].to_vec());

    let repeat = instructions.len();
    let mut cur_pos = "AAA";
    let mut index = 0;

    while cur_pos != "ZZZ" {
        if instructions[index % repeat] == 'L' {
            cur_pos = &possibilities.get(cur_pos).unwrap().L;
        } else if instructions[index % repeat] == 'R' {
            cur_pos = &possibilities.get(cur_pos).unwrap().R;
        }

        index += 1;
    }

    index.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn it_works() {
        let result = part1(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "2");
    }
}
