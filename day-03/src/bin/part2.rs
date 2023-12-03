use std::collections::{HashMap, HashSet};

fn main() {
    let input = parse_input(include_str!("input.txt"));
    let output = part1(input);
    println!("{}", output);
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .split_whitespace()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn adjacent_to_gear(lines: &[Vec<char>], y: usize, x: usize) -> Option<(usize, usize)> {
    let directions: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (0, -1),
        (1, 1),
        (1, 0),
        (0, 1),
        (1, -1),
        (-1, 1),
    ];

    let mut gear: Option<(usize, usize)> = None;

    directions.iter().any(|(y_transform, x_transform)| {
        let new_y = match y.checked_add_signed(*y_transform) {
            None => return false,
            Some(usize) => usize,
        };
        let new_x = match x.checked_add_signed(*x_transform) {
            None => return false,
            Some(usize) => usize,
        };
        if new_y >= lines.len() || new_x >= lines[new_y].len() {
            return false;
        }
        if lines[new_y][new_x] == '*' {
            gear = Some((new_y, new_x));
            true
        } else {
            false
        }
    });
    gear
}

fn part1(lines: Vec<Vec<char>>) -> usize {
    let mut y = 0;
    let mut x = 0;
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut gear_number_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    while y != lines.len() {
        if lines[y][x].is_numeric() && !seen.contains(&(y, x)) {
            let mut number = String::from("");
            let mut gear_coords: Option<(usize, usize)> = None;

            while x < lines[y].len() && lines[y][x].is_numeric() {
                number.push(lines[y][x]);
                seen.insert((y, x));
                let gear = adjacent_to_gear(&lines, y, x);
                match gear {
                    None => (),
                    Some(gear) => gear_coords = Some(gear),
                }

                x += 1;
            }

            let parsed_number: usize = number.parse::<usize>().unwrap();

            match gear_coords {
                None => (),
                Some(gear) => {
                    gear_number_map
                        .entry(gear)
                        .and_modify(|x| x.push(parsed_number))
                        .or_insert(vec![parsed_number]);
                }
            };
        }

        if x < lines[y].len() - 1 {
            x += 1;
        } else {
            x = 0;
            y += 1;
        }
    }
    gear_number_map.values().fold(0, |acc, numbers| {
        if numbers.len() == 2 {
            acc + numbers.iter().product::<usize>()
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = include_str!("example.txt");
        let result = part1(parse_input(example));
        assert_eq!(result, 467835);
    }
}
