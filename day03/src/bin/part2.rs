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
    let out = part2(input);
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

fn part2(input: &str) -> String {
    let mut out = 0;

    let mut part_numbers = find_numbers(input);

    for n in part_numbers.as_slice() {
        println!("{:?}", n);
    }

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let mut is_gear = false;
            match c {
                '*' => is_gear = true,
                _ => (),
            };

            if is_gear {
                let mut adjecent_gears: Vec<&PartNumber> = Vec::new();

                for mut part_number in part_numbers.as_mut_slice() {
                    if part_number.x <= (x + 1) as i32
                        && part_number.x + part_number.len >= x as i32
                        && part_number.y <= (y + 1) as i32
                        && part_number.y >= (y - 1) as i32
                        && !part_number.is_collected
                    {
                        part_number.is_collected = true;
                        adjecent_gears.push(part_number);
                    }
                }

                if adjecent_gears.len() == 2 {
                    out += adjecent_gears.iter().map(|gear| gear.num ).product::<i32>();
                }
            }
        }
    }

    out.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;
    #[test]
    fn it_works() {
        let result = part2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "467835");
    }
}
