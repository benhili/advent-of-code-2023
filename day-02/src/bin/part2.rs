use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = sum_minimum_required_cubes(input);
    dbg!(output);
}

/// Calculates the power of all cubes required in a game
///
/// # Examples
/// ```
/// let game = "Game 1: red 5, blue 2";
/// let answer = part_02::get_power_of_minimum_required_cubes(game)
///
/// assert_eq!(10, answer);
/// // The product of 5 and 2
/// ```
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

    cube_totals.values().product()
}

fn sum_minimum_required_cubes(input: &str) -> u32 {
    let games = input.lines();
    games.map(get_power_of_minimum_required_cubes).sum()
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

    #[test]
    fn it_gets_the_power_of_minimum_required_cubes() {
        let example = "Game 1: 5 red, 7 blue; 6 green, 1 red";
        let result = get_power_of_minimum_required_cubes(example);
        assert_eq!(result, 210);
    }
}
