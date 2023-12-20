fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn part2(input: &str) -> String {
    let output = input
        .lines()
        .map(|line|{
            let mut it = (0..line.len())
                .filter_map(|index|{
                    let reducedLine = &line[index..];
                    let result = if reducedLine.starts_with("one"){
                        '1'
                    } else if reducedLine.starts_with("two"){
                        '2'
                    } else if reducedLine.starts_with("three"){
                        '3'
                    } else if reducedLine.starts_with("four"){
                        '4'
                    } else if reducedLine.starts_with("five"){
                        '5'
                    } else if reducedLine.starts_with("six"){
                        '6'
                    } else if reducedLine.starts_with("seven"){
                        '7'
                    } else if reducedLine.starts_with("eight"){
                        '8'
                    } else if reducedLine.starts_with("nine"){
                        '9'
                    } else {
                        reducedLine.chars().next().unwrap()
                    };
                    result.to_digit(10)
                });
            let first = 
                it.next().expect("should be a number");
            let last = it.last();
            match last {
                Some(num) => format!("{first}{num}"),
                None => format!("{first}{first}"),
            }
            .parse::<u32>()
            .expect("should be a number")
    })
    .sum::<u32>();
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, "281".to_string());
    }
}
