#[derive(Debug)]
struct Tile {
    x: i32,
    y: i32,
    char: char,
}

fn main() {
    let input = include_str!("./input10.txt");
    let out = part1(input);
    println!("{out}");
}

fn parse_input(input: &str) -> Vec<Tile> {
    input
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .map(|(pos, char)| Tile {
                    x: pos as i32,
                    y: line_index as i32,
                    char: char,
                })
                .collect::<Vec<Tile>>()
        })
        .collect()
}

fn find_farthest(map: &mut Vec<Tile>) -> i32 {
    let start_tile = map.iter().find(|t| t.char == 'S').unwrap();

    let mut adjecent_tiles: Vec<&Tile> = Vec::new();
    match map
        .iter()
        .find(|t| t.x == start_tile.x && t.y == start_tile.y - 1 && t.char != '.')
    {
        Some(tile) => adjecent_tiles.push(tile),
        None => (),
    };
    match map
        .iter()
        .find(|t| t.x == start_tile.x + 1 && t.y == start_tile.y && t.char != '.')
    {
        Some(tile) => adjecent_tiles.push(tile),
        None => (),
    };
    match map
        .iter()
        .find(|t| t.x == start_tile.x && t.y == start_tile.y + 1 && t.char != '.')
    {
        Some(tile) => adjecent_tiles.push(tile),
        None => (),
    };
    match map
        .iter()
        .find(|t| t.x == start_tile.x - 1 && t.y == start_tile.y && t.char != '.')
    {
        Some(tile) => adjecent_tiles.push(tile),
        None => (),
    };

    let mut count = 1;

    let mut prev = *adjecent_tiles.first().unwrap();
    let mut cur = start_tile;
    let mut next = *adjecent_tiles.last().unwrap();

    while next.char != 'S' {
        count += 1;
        prev = cur;
        cur = next;

        next = match cur.char {
            '|' => map
                .iter()
                .find(|t| t.y == cur.y + (cur.y - prev.y) && t.x == cur.x)
                .unwrap(),
            '-' => map
                .iter()
                .find(|t| t.y == cur.y && t.x == cur.x + (cur.x - prev.x))
                .unwrap(),
            'L' => map
                .iter()
                .find(|t| {
                    t.y == cur.y + (cur.y - prev.y) - 1 && t.x == cur.x + (cur.x - prev.x) + 1
                })
                .unwrap(),
            'J' => map
                .iter()
                .find(|t| {
                    t.y == cur.y + (cur.y - prev.y) - 1 && t.x == cur.x + (cur.x - prev.x) - 1
                })
                .unwrap(),
            '7' => map
                .iter()
                .find(|t| t.y == cur.y + (cur.y - prev.y) + 1 && t.x == cur.x + (cur.x - prev.x) - 1)
                .unwrap(),
            'F' => map
                .iter()
                .find(|t| t.y == cur.y + (cur.y - prev.y) + 1 && t.x == cur.x + (cur.x - prev.x) + 1)
                .unwrap(),
            'S' => break,
            _ => panic!("should not happen!"),
        };
    }

    count / 2
}

fn part1(input: &str) -> String {
    let mut map = parse_input(input);

    find_farthest(&mut map).to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn it_works() {
        let result = part1(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn it_works2() {
        let result = part1(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, "4");
    }
}
