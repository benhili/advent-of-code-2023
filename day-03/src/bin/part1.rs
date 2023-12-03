use std::collections::HashSet;

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

fn adjacent_to_symbol(lines: &[Vec<char>], y: usize, x: usize) -> bool {
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

        lines[new_y][new_x] != '.' && !lines[new_y][new_x].is_numeric()
    })
}

fn part1(lines: Vec<Vec<char>>) -> usize {
    let mut y = 0;
    let mut x = 0;
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut valid_numbers: Vec<usize> = vec![];

    while y != lines.len() {
        if lines[y][x].is_numeric() && !seen.contains(&(y, x)) {
            let mut number = String::from("");
            let mut valid = false;

            while x < lines[y].len() && lines[y][x].is_numeric() {
                number.push(lines[y][x]);
                seen.insert((y, x));
                if adjacent_to_symbol(&lines, y, x) {
                    valid = true
                }
                x += 1;
            }

            if valid {
                valid_numbers.push(number.parse::<usize>().unwrap());
            }
        }

        if x < lines[y].len() - 1 {
            x += 1;
        } else {
            x = 0;
            y += 1;
        }
    }

    valid_numbers.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = include_str!("example.txt");
        let result = part1(parse_input(example));
        assert_eq!(result, 4361);
    }
}
