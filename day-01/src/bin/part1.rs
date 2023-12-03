fn main() {
    let input = include_str!("input.txt");
    let output = sum_first_and_last_digits(input).unwrap();
    dbg!(output);
}

// given a string of chars and digits combines the first and last digits
// in the string eg er1ekfjn34lj9 would return 19
fn combine_first_and_last_digits(input: &str) -> u32 {
    let first = input.chars().find_map(|c| c.to_digit(10)).unwrap();
    let last = input.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    10 * first + last
}

fn sum_first_and_last_digits(input: &str) -> Option<u32> {
    Some(input.lines().map(combine_first_and_last_digits).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = include_str!("example.txt");
        let result = sum_first_and_last_digits(example).unwrap();
        assert_eq!(result, 142);
    }
}
