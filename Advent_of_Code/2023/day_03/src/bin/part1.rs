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
        //Add the numbers only touching a symbol (even diagonally) in the following grid:
        let result = part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        //From the above grid, we have 467, 35, 633, 617, 592, 755, 664, 598 touching a symbol.
        //Adding them up gives 4361.
        assert_eq!(result, "4361".to_string());
    }
}
