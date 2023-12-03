#[derive(Debug)]
struct PartNumber {
    num: i32,
    x: i32,
    y: i32,
    len: i32,
    is_collected: bool,
}

fn main() {
    let input = include_str!("./input3.txt");
    let out = part1(input);
    println!("{out}");
}

fn find_numbers(input_file: &str) -> Vec<PartNumber> {
    let mut numbers: Vec<PartNumber> = Vec::new();
    for (y, line) in input_file.lines().enumerate() {
        let mut num_start = 0;
        let mut is_parsing = false;
        for (x, c) in line.chars().enumerate() {
            if c.is_digit(10) && !is_parsing {
                num_start = x;
                is_parsing = true;
            } else if !c.is_digit(10) && is_parsing {
                is_parsing = false;
                numbers.push(PartNumber {
                    num: line[num_start..x].parse::<i32>().unwrap(),
                    x: num_start as i32,
                    y: y as i32,
                    len: (x - num_start) as i32,
                    is_collected: false,
                })
            } else if line.len() == x+1 && is_parsing {
                is_parsing = false;
                numbers.push(PartNumber {
                    num: line[num_start..x+1].parse::<i32>().unwrap(),
                    x: num_start as i32,
                    y: y as i32,
                    len: (x+1 - num_start) as i32,
                    is_collected: false,
                })
            }
        }
    }

    numbers
}

fn part1(input: &str) -> String {
    let mut out = 0;

    let mut part_numbers = find_numbers(input);

    for n in part_numbers.as_slice() {
        println!("{:?}", n);
    }

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut is_symbol = false;
            match c {
                '.' => is_symbol = false,
                ref x if !x.is_digit(10) => is_symbol = true,
                _ => (),
            };

            if is_symbol {
                for mut part_number in part_numbers.as_mut_slice() {
                    if part_number.x <= (x + 1) as i32
                        && part_number.x + part_number.len >= x as i32
                        && part_number.y <= (y + 1) as i32
                        && part_number.y >= (y - 1) as i32
                        && !part_number.is_collected
                    {
                        part_number.is_collected = true;
                        out += part_number.num;
                        println!("{}", part_number.num);
                    }
                }
            }
        }
    }

    out.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn it_works() {
        let result = part1(
            "123.234
...*...
345.456",
        );
        assert_eq!(result, "4361");
    }
}
