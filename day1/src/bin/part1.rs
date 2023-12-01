fn main() {
    let input = include_str!("./input1.txt");
    let out = part1(input);
    println!("{out}");
}

fn part1(input: &str) -> String {
    //let input = String::from(&str);
    let mut res = 0;

    for line in input.lines() {
        let mut digits_in_line: Vec<String> = Vec::new();
        for character in line.chars() {
            if character.is_numeric() {
                digits_in_line.push(character.to_string());
            }
        }
        let s = format!("{}{}", digits_in_line.first().unwrap(), digits_in_line.last().unwrap());

        res += s.parse::<i32>().unwrap();
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, "142");
    }
}
