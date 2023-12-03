use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = sum_minimum_required_cubes(input);
    dbg!(output);
}

fn get_power_of_minimum_required_cubes(game: &str) -> u32 {
    let cube_sets = game.split(':').nth(1).unwrap().split(';');
    let mut cube_totals: HashMap<&str, u32> = HashMap::new();

    for set in cube_sets {
        let cubes = set.split(',');
        for cube in cubes {
            let count = cube
                .trim()
                .split(' ')
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let color = cube.trim().split(' ').nth(1).unwrap();
            cube_totals
                .entry(color)
                .and_modify(|curr| {
                    *curr = cmp::max(*curr, count);
                })
                .or_insert(count);
        }
    }

    return cube_totals.values().product();
}

fn sum_minimum_required_cubes(games: &str) -> u32 {
    return games.lines().map(get_power_of_minimum_required_cubes).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = include_str!("example.txt");
        let result = sum_minimum_required_cubes(example);
        assert_eq!(result, 8);
    }
}
