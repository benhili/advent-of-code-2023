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
    let output = sum_first_and_last_digits_or_number_symbols(input).unwrap();
    dbg!(output);
}

fn get_first_digit_or_number_symbol(slice: &str) -> Option<u32> {
    LUT.entries()
        .find(|(k, _)| slice.starts_with(*k))
        .map(|(_, v)| *v)
        .or_else(|| slice.chars().next().unwrap().to_digit(10))
}

// for a given string eg esix12mkwearh8 return first and last digit
// or number string eg "six" as an integer 68
fn combine_first_and_last_digit_or_number_symbol(line: &str) -> u32 {
    let first = (0..line.len())
        .find_map(|i| get_first_digit_or_number_symbol(&line[i..]))
        .unwrap();

    let last = (0..line.len())
        .rev()
        .find_map(|i| get_first_digit_or_number_symbol(&line[i..]))
        .unwrap();

    10 * first + last
}

fn sum_first_and_last_digits_or_number_symbols(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(combine_first_and_last_digit_or_number_symbol)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_example() {
        let example = include_str!("example.txt");
        let result = sum_first_and_last_digits_or_number_symbols(example).unwrap();
        assert_eq!(result, 142);
    }
}
