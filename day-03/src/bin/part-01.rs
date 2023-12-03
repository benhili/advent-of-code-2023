fn main() {
    let input = include_str!("input.txt");
    let output = part1();
    dbg!(output);
}

fn part1() {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let example = include_str!("example.txt");
        let result = part1();
        assert_eq!(result, 8);
    }
}
