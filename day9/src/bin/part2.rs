fn main() {
    let input = include_str!("./input9.txt");
    let out = part2(input);
    println!("{out}");
}

fn calculate(line: Vec<i32>) -> i32 {
    let step = line.windows(2)
                .into_iter()
                .map(|num_pair| num_pair[1] - num_pair[0])
                .collect::<Vec<i32>>();

    let mut inc = step.first().unwrap().clone();

    if inc != 0 || step.last().unwrap().clone() != 0{
        inc -= calculate(step);
    }

    inc
}

fn part2(input: &str) -> String {
    let split_input: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let mut out: i32 = 0;
    for line in split_input.iter() {
        let x: i32 = calculate(line.to_vec());
        let y = line.first().unwrap() - x;
        out += y;
    }

    out.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;
    #[test]
    fn it_works() {
        let result = part2(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, "2");
    }
}
