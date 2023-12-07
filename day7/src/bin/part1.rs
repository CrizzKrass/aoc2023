use std::{cmp::Ordering, collections::{HashMap}, panic};

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
}

#[derive(Debug)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

fn main() {
    let input = include_str!("./input7.txt");
    let out = part1(input);
    println!("{out}");
}

fn match_cards(card: char) -> u8 {
    match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("fuck off"),
    }
}

fn match_hand_type(hand_type: HandType) -> u8 {
    match hand_type {
        HandType::HighCard => 0,
        HandType::OnePair => 1,
        HandType::TwoPair => 2,
        HandType::ThreeOfAKind => 3,
        HandType::FullHouse => 4,
        HandType::FourOfAKind => 5,
        HandType::FiveOfAKind => 6,
    }
}

fn parse_string(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let hand_data: Vec<&str> = line.trim().split_ascii_whitespace().collect();
            Hand {
                cards: hand_data[0].to_string(),
                bid: hand_data[1].parse::<u32>().unwrap(),
            }
        })
        .collect()
}

fn get_hand_kind(hand_cards: &str) -> HandType {
    let hand:HashMap<u8, u8> = hand_cards.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(match_cards(c)).or_insert(0) += 1;
        acc
    });

    match hand.iter().count() {
        1 => HandType::FiveOfAKind,
        2 => if hand.iter().any(|(_,v)| *v == 4) {
            HandType::FourOfAKind
        } else {
            HandType::FullHouse
        },
        3 => if hand.iter().any(|(_,v)| *v == 3) {
            HandType::ThreeOfAKind
        } else {
            HandType::TwoPair
        },
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => panic!("fuck off nr.2")
    }
}

fn compare(a: &Hand, b: &Hand) -> Ordering {
    let hand_a = get_hand_kind(&a.cards);
    let hand_b = get_hand_kind(&b.cards);
    let test = match_hand_type(hand_a).cmp(&match_hand_type(hand_b));
    
    match test {
        Ordering::Less => Ordering::Less,
        Ordering::Greater => Ordering::Greater,
        Ordering::Equal => {
            for index in 0..5 {
                let res = match_cards(a.cards.chars().nth(index).unwrap()).cmp(&match_cards(b.cards.chars().nth(index).unwrap()));
                if  res != Ordering::Equal {
                    return res;
                }
            }

            Ordering::Equal
        }
    }
}

fn part1(input: &str) -> String {
    let mut hands = parse_string(input);

    hands.sort_by(|a, b| compare(a, b));

    let values: Vec<u32> = hands.iter().enumerate().map(|(index, hand)| {
        hand.bid * (index+1) as u32
    }).collect();

    let mut sum: u32 = 0;
    for i in values {
        sum += i;
    }

    sum.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn it_works() {
        let result = part1(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        );
        assert_eq!(result, "5905");
    }
}
