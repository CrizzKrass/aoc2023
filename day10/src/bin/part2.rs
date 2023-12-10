#[derive(Debug)]
struct Tile {
    x: i32,
    y: i32,
    char: char,
}

fn main() {
    let input = include_str!("./input10.txt");
    let out = part2(input);
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

fn get_enclosed(map: &mut Vec<Tile>) -> i32 {
    let start_tile = map.iter().find(|t| t.char == 'S').unwrap();

    let mut adjecent_tiles: Vec<&Tile> = Vec::new();
    match map.iter().find(|t| {
        t.x == start_tile.x && t.y == start_tile.y - 1 && ['|', 'F', '7'].contains(&t.char)
    }) {
        Some(tile) => adjecent_tiles.push(tile),
        None => (),
    };
    match map.iter().find(|t| {
        t.x == start_tile.x + 1 && t.y == start_tile.y && ['-', 'J', '7'].contains(&t.char)
    }) {
        Some(tile) => adjecent_tiles.push(tile),
        None => (),
    };
    match map.iter().find(|t| {
        t.x == start_tile.x && t.y == start_tile.y + 1 && ['|', 'J', 'L'].contains(&t.char)
    }) {
        Some(tile) => adjecent_tiles.push(tile),
        None => (),
    };
    match map.iter().find(|t| {
        t.x == start_tile.x - 1 && t.y == start_tile.y && ['-', 'F', 'L'].contains(&t.char)
    }) {
        Some(tile) => adjecent_tiles.push(tile),
        None => (),
    };

    let mut tiles_of_loop: Vec<&Tile> = Vec::new();

    let mut prev = *adjecent_tiles.first().unwrap();
    let mut cur = start_tile;
    tiles_of_loop.push(cur);
    let mut next = *adjecent_tiles.last().unwrap();

    while next.char != 'S' {
        prev = cur;
        cur = next;
        tiles_of_loop.push(cur);

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
                .find(|t| {
                    t.y == cur.y + (cur.y - prev.y) + 1 && t.x == cur.x + (cur.x - prev.x) - 1
                })
                .unwrap(),
            'F' => map
                .iter()
                .find(|t| {
                    t.y == cur.y + (cur.y - prev.y) + 1 && t.x == cur.x + (cur.x - prev.x) + 1
                })
                .unwrap(),
            'S' => break,
            _ => panic!("should not happen!"),
        };
    }

    let start_tile_char = match adjecent_tiles.iter().fold(0, |acc, t| {
        if start_tile.x - t.x > 0 {
            return acc + 8;
        } else if start_tile.x - t.x < 0 {
            return acc + 2;
        } else if start_tile.y - t.y > 0 {
            return acc + 1;
        } else if start_tile.y - t.y < 0 {
            return acc + 4;
        }
        panic!("help");
    }) {
        3 => 'L',
        5 => '|',
        6 => 'F',
        9 => 'J',
        10 => '-',
        12 => '7',
        _ => panic!("asdf"),
    };

    dbg!(start_tile_char);

    let mut count = 0;
    let mut tiles_inside = 0;
    let mut start_char: char = ' ';
    for tile in map.iter() {
        if tile.x == 0 {
            count = 0;
        }

        if tiles_of_loop
            .iter()
            .find(|t| t.x == tile.x && t.y == tile.y)
            .is_some()
        {
            if tile.char == '|'
                || (tile.char == 'S' && adjecent_tiles.iter().all(|t| t.char == '|'))
            {
                count += 1;
            }
            if tile.char == 'F'
                || tile.char == 'L'
                || (['F', 'L'].contains(&start_tile_char) && tile.char == 'S')
            {
                start_char = if tile.char != 'S' {
                    tile.char
                } else {
                    dbg!(tile.char);
                    start_tile_char
                }
            }
            if ((tile.char == 'J' || (start_tile_char == 'J' && tile.char == 'S'))
                && start_char == 'F')
                || ((tile.char == '7' || (start_tile_char == '7' && tile.char == 'S'))
                    && start_char == 'L')
            {
                dbg!(tile);
                dbg!(start_char);
                count += 1;
                start_char = ' ';
            }
        } else if count % 2 == 1 {
            tiles_inside += 1;
        }
    }

    tiles_inside as i32
}

fn part2(input: &str) -> String {
    let mut map = parse_input(input);

    get_enclosed(&mut map).to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;
    #[test]
    fn it_works() {
        let result = part2(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
        );
        assert_eq!(result, "4");
    }

    #[test]
    fn it_works2() {
        let result = part2(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
        );
        assert_eq!(result, "8");
    }

    #[test]
    fn it_works3() {
        let result = part2(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
        );
        assert_eq!(result, "10");
    }
}
