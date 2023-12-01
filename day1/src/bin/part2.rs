fn main() {
    let input = include_str!("./input1.txt");
    let out = part2(input);
    println!("{out}");
}

fn part2(input: &str) -> String {
    //let input = String::from(&str);
    let mut res = 0;

    for line in input.lines() {
        let mut digits_in_line: Vec<String> = Vec::new();
        let mut test = String::new();
        for c in line.chars() {
            if c.is_numeric() {
                digits_in_line.push(c.to_string());
                test = String::new();
            } else {
                test.push(c);
            }

            match test {
                ref x
                    if test.chars().count() >= 3
                        && x[test.chars().count() - 3..].to_string() == "one".to_string() =>
                {
                    digits_in_line.push("1".to_string());
                }
                ref x
                    if test.chars().count() >= 3
                        && x[test.chars().count() - 3..].to_string() == "two".to_string() =>
                {
                    digits_in_line.push("2".to_string());
                }
                ref x
                    if test.chars().count() >= 5
                        && x[test.chars().count() - 5..].to_string() == "three".to_string() =>
                {
                    digits_in_line.push("3".to_string());
                }
                ref x
                    if test.chars().count() >= 4
                        && x[test.chars().count() - 4..].to_string() == "four".to_string() =>
                {
                    digits_in_line.push("4".to_string());
                }
                ref x
                    if test.chars().count() >= 4
                        && x[test.chars().count() - 4..].to_string() == "five".to_string() =>
                {
                    digits_in_line.push("5".to_string());
                }
                ref x
                    if test.chars().count() >= 3
                        && x[test.chars().count() - 3..].to_string() == "six".to_string() =>
                {
                    digits_in_line.push("6".to_string());
                }
                ref x
                    if test.chars().count() >= 5
                        && x[test.chars().count() - 5..].to_string() == "seven".to_string() =>
                {
                    digits_in_line.push("7".to_string());
                }
                ref x
                    if test.chars().count() >= 5
                        && x[test.chars().count() - 5..].to_string() == "eight".to_string() =>
                {
                    digits_in_line.push("8".to_string());
                }
                ref x
                    if test.chars().count() >= 4
                        && x[test.chars().count() - 4..].to_string() == "nine".to_string() =>
                {
                    digits_in_line.push("9".to_string());
                }

                _ => (),
            }
        }

        let s = format!(
            "{}{}",
            digits_in_line.first().unwrap(),
            digits_in_line.last().unwrap()
        );

        res += s.parse::<i32>().unwrap();
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;
    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
