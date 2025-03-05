fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn part2(input: &str) -> String {
    let output = "TODO";
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("");
        assert_eq!(result, "".to_string());
    }
}
