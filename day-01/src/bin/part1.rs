fn main() {
    let input = include_str!("input.txt");
    let output = part_one(input).unwrap();
    dbg!(output);
}

fn parse_line(input: &str) -> u32 {
    let first = input.chars().find_map(|c| c.to_digit(10)).unwrap();
    let last = input.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    return 10 * first + last;
}

fn part_one(input: &str) -> Option<u32> {
    return Some(input.lines().map(parse_line).sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = include_str!("example.txt");
        let result = part_one(example).unwrap();
        assert_eq!(result, 142);
    }
}
