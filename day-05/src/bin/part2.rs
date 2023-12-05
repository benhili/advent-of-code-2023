use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("input.txt");
    let output = part2(input);
    dbg!(output);
}

struct RangeMap {
    source_range_start: i64,
    dest_range_start: i64,
    range_len: i64,
}

impl RangeMap {
    fn get_dest_from_source(&self, source: i64) -> Option<i64> {
        let diff_from_range_start = source - self.source_range_start;
        let in_range = 0 <= diff_from_range_start && diff_from_range_start <= self.range_len;
        if in_range {
            Some(self.dest_range_start + diff_from_range_start)
        } else {
            None
        }
    }
}

fn map_over_range_maps(vec_of_range_maps: &Vec<Vec<RangeMap>>, seed: i64) -> i64 {
    let mut pointer = seed;

    for range_maps in vec_of_range_maps {
        pointer = range_maps
            .iter()
            .find_map(|range_map| range_map.get_dest_from_source(pointer))
            .unwrap_or(pointer);
    }

    pointer
}

fn str_nums_to_vec_nums(nums: &str) -> Vec<i64> {
    nums.split_whitespace()
        .map(|num_str| num_str.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn parse(input: &str) -> (Vec<i64>, Vec<Vec<RangeMap>>) {
    let mut chunks = input.split("\n\n");
    let mut vec_of_range_maps: Vec<Vec<RangeMap>> = Vec::new();

    let seed_str = chunks.next().unwrap().split(": ").nth(1).unwrap();
    let seeds = str_nums_to_vec_nums(seed_str);

    for chunk in chunks {
        let mut range_maps: Vec<RangeMap> = Vec::new();
        let lines = chunk.lines();
        // discard title line
        for range_nums in lines.skip(1) {
            let data = str_nums_to_vec_nums(range_nums);
            range_maps.push(RangeMap {
                dest_range_start: data[0],
                source_range_start: data[1],
                range_len: data[2],
            })
        }
        vec_of_range_maps.push(range_maps)
    }
    (seeds, vec_of_range_maps)
}

fn part2(input: &str) -> i64 {
    let (seeds, vec_of_range_maps) = parse(input);

    let actual_seeds: Vec<i64> = seeds
        .chunks(2)
        .flat_map(|ch| (ch[0]..ch[0] + ch[1]))
        .collect();

    println!("seeds computed");
    println!("dont care brute force 🤪");

    actual_seeds
        .par_iter()
        .map(|seed| map_over_range_maps(&vec_of_range_maps, *seed))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = include_str!("example.txt");
        let result = part2(example);
        assert_eq!(result, 46);
    }

    #[test]
    fn it_maps_seed_to_soil() {
        let range_maps = vec![vec![
            RangeMap {
                source_range_start: 98,
                dest_range_start: 50,
                range_len: 2,
            },
            RangeMap {
                source_range_start: 50,
                dest_range_start: 52,
                range_len: 48,
            },
        ]];

        assert_eq!(10, map_over_range_maps(&range_maps, 10));
        assert_eq!(51, map_over_range_maps(&range_maps, 99));
    }
}
