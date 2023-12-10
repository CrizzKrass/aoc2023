use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    chosen_numbers: Vec<i32>,
    count: i32,
    matching: i32,
}
fn main() {
    let input = include_str!("./input4.txt");
    let out = part2(input);
    println!("{out}");
}

fn parse_cards(input: &str) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::new();

    for line in input.lines() {
        let split_line: Vec<&str> = line.split(":").collect();
        let id = split_line[0].replace("Card", "").trim().parse::<i32>().unwrap();
        let game_data: Vec<&str> = split_line[1].split("|").collect();
        let winning_numbers: Vec<i32> = game_data[0].trim().split(" ").filter(|str| *str != "").map(|num| {
            match num.parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    println!("Err");
                    0
                },
            }
        }).collect();

        let chosen_numbers: Vec<i32> = game_data[1].trim().split(" ").filter(|str| *str != "").map(|num| {
            match num.parse::<i32>() {
                Ok(n) => n,
                Err(_) => {
                    println!("Err");
                    0
                },
            }
        }).collect();

        cards.push(Card { id, winning_numbers, chosen_numbers, count: 1, matching: 0 });
    }

    cards
}


fn part2(input: &str) -> String {
    let mut cards = parse_cards(input);

    let mut card_info: HashMap<i32, Card> = HashMap::new();

    for mut card in cards.as_mut_slice() {
        let mut matching_numbers = 0;
        for winning_num in &card.winning_numbers {
            if card.chosen_numbers.contains(&winning_num) {
                matching_numbers += 1;
            }
        }
        card.matching = matching_numbers;
        card_info.insert(card.id, card.clone());
    }

    let mut cards = cards.clone();

    for mut card in cards.as_mut_slice() {
        for i in 1..card.id {
            if card_info.get_key_value(&i).is_some() {
                let info = card_info.get(&i).unwrap();

                if i < card.id && i+info.matching >= card.id {
                    card.count += info.count;
                    let mut x = card_info.get_mut(&card.id).unwrap();
                    x.count = card.count;
                }
            }
        }
    }

    let mut out = 0;
    for card in cards {
        out += card.count;
    }

    out.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;
    #[test]
    fn it_works() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, "30");
    }
}
