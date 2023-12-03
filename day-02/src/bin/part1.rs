use regex::Regex;
use std::collections::HashMap;

fn main() {
    let cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let input = include_str!("input.txt");
    let output = sum_possible_game_ids(input, cubes);
    dbg!(output);
}

// for a given game: Game 1: red 13, blue 15; returns the game ID 1
// if the number of cubes of each color used are less than the total
// cubes provided in the map otherwise returns 0
fn get_game_id_if_valid(game: &str, cube_totals: &HashMap<&str, i32>) -> u32 {
    let re = Regex::new(r"Game (\d+)").unwrap();
    let id = re.captures(game.split(':').next().unwrap()).unwrap()[1]
        .parse::<u32>()
        .unwrap();

    let cube_sets = game.split(':').nth(1).unwrap().split(';');

    for set in cube_sets {
        let cubes = set.split(',');
        for cube in cubes {
            let count = cube
                .trim()
                .split(' ')
                .next()
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let color = cube.trim().split(' ').nth(1).unwrap();
            let total = *cube_totals.get(color).unwrap();

            if total < count {
                return 0;
            }
        }
    }

    id
}

fn sum_possible_game_ids(games: &str, cubes: HashMap<&str, i32>) -> u32 {
    return games
        .lines()
        .map(|game| get_game_id_if_valid(game, &cubes))
        .sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cubes = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);
        let example = include_str!("example.txt");
        let result = sum_possible_game_ids(example, cubes);
        assert_eq!(result, 8);
    }
}
