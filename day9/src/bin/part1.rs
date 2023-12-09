fn main() {
    let input = include_str!("./input9.txt");
    let out = part1(input);
    println!("{out}");
}

fn calculate(line: Vec<i32>) -> i32 {
    let step = line.windows(2)
                .into_iter()
                .map(|num_pair| num_pair[1] - num_pair[0])
                .collect::<Vec<i32>>();

    let mut inc = step.last().unwrap().clone();

    if inc != 0 {
        inc += calculate(step);
    }

    inc
}

fn part1(input: &str) -> String {
    let split_input: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut out = 0;
    for line in split_input.iter() {
        let x = calculate(line.to_vec());
        out += line.last().unwrap() + x;
    }

    out.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn it_works() {
        let result = part1(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, "114");
    }
}
