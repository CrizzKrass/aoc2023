#[derive(Debug)]
struct Galaxy {
    x: i32,
    y: i32,
}

fn main() {
    let input = include_str!("./input11.txt");
    let out = part1(input);
    println!("{out}");
}

fn parse_input(input: &str) -> Vec<Galaxy> {
    input
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c != '.')
                .map(|(i, _)| Galaxy {
                    x: i as i32,
                    y: line_index as i32,
                })
                .collect::<Vec<Galaxy>>()
        })
        .collect()
}

fn expand_galaxies(mut galaxies: Vec<Galaxy>) -> Vec<Galaxy> {
    let width = galaxies.iter().max_by(|g1, g2| g1.x.cmp(&g2.x)).unwrap().x;
    let height = galaxies.iter().max_by(|g1, g2| g1.y.cmp(&g2.y)).unwrap().y;

    let mut inc: Vec<i32> = Vec::new();
    for i in 0..width {
        if galaxies.iter().all(|g| g.x != i) {
            inc.push(i);
        }
    }

    for galaxy in galaxies.iter_mut() {
        let inc = inc.iter().filter(|i| i < &&galaxy.x).count();
        galaxy.x += inc as i32;
    }

    let mut inc: Vec<i32> = Vec::new();
    for i in 0..height {
        if galaxies.iter().all(|g| g.y != i) {
            inc.push(i);
        }
    }

    for galaxy in galaxies.iter_mut() {
        let inc = inc.iter().filter(|i| i < &&galaxy.y).count();
        galaxy.y += inc as i32;
    }
    galaxies
}

fn part1(input: &str) -> String {
    let galaxies = parse_input(input);

    let galaxies = expand_galaxies(galaxies);

    let mut res = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for galaxy2 in galaxies[i + 1..].iter() {
            let distance = (galaxy.x - galaxy2.x).abs() + (galaxy.y - galaxy2.y).abs();
            res += distance;
        }
    }

    res.to_string()
}

#[cfg(test)]
mod tests {
    use crate::part1;
    #[test]
    fn it_works() {
        let result = part1(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, "374");
    }
}
