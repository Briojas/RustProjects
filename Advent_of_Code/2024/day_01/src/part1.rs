fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

pub fn part1(input: &str) -> String {
    let output = "TODO";
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("");
        assert_eq!(result, "".to_string());
    }
}
