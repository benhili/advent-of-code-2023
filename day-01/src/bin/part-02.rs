use phf::phf_map;

static LUT: phf::Map<&str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

fn main() {
    let input = include_str!("input.txt");
    let output = part_two(input).unwrap();
    dbg!(output);
}

fn parse_slice(slice: &str) -> Option<u32> {
    return LUT
        .entries()
        .find(|(k, _)| slice.starts_with(*k))
        .map(|(_, v)| *v)
        .or_else(|| slice.chars().next().unwrap().to_digit(10));
}

fn parse_line(line: &str) -> u32 {
    let first = (0..line.len())
        .find_map(|i| parse_slice(&line[i..]))
        .unwrap();

    let last = (0..line.len())
        .rev()
        .find_map(|i| parse_slice(&line[i..]))
        .unwrap();

    return 10 * first + last;
}

fn part_two(input: &str) -> Option<u32> {
    return Some(input.lines().map(parse_line).sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = include_str!("example.txt");
        let result = part_two(example).unwrap();
        assert_eq!(result, 142);
    }
}
