fn main() {
    let input = include_str!("./input2.txt");
    let out = part1(input);
    println!("{out}");
}

fn part1(input: &str) -> String {
    let mut out = 0;
    for line in input.lines() {
        let mut possible = true;

        let colon_pos = line.find(":").unwrap();
        let id = match line[5..colon_pos].parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Error occured");
                continue;
            }
        };

        let game_data: Vec<&str> = line[colon_pos + 1..line.len()].split(";").collect();

        //go through data, get dice count
        for rounds in game_data {
            let round_data: Vec<&str> = rounds.split(",").collect();
            for data in round_data {
                let dice_count: Vec<&str> = data.split(" ").collect();
                let dice_count = dice_count[1].parse::<i32>().unwrap();
                if data.contains("blue") {
                    if dice_count > 14 {
                        possible = false;
                    }
                }
                if data.contains("red") {
                    if dice_count > 12 {
                        possible = false;
                    }
                }
                if data.contains("green") {
                    if dice_count > 13 {
                        possible = false;
                    }
                }
            }
        }

        if possible {
            out += id;
        } else {
            println!("{id}");
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
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8");
    }
}
