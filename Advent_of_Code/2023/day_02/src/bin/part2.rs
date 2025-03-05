fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn part2(_input: &str) -> String {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut result = 0;

    let _output = _input
        .lines()
        .for_each(|line|{
            let mut game = line.split(": ");
            let _game_id = game.next().unwrap();
            let game_id_number = _game_id.split_whitespace().nth(1).unwrap().parse::<i32>().unwrap();
            let game = game.next().unwrap();
            let colors = game.split("; ");
            let mut red_highest = 0;
            let mut green_highest = 0;
            let mut blue_highest = 0;
            for color in colors {
                let color = color.split(", ");
                for c in color {
                    let mut c = c.split_whitespace();
                    let count = c.next().unwrap().parse::<i32>().unwrap();
                    let color = c.next().unwrap();
                    match color {
                        "red" => if count > red_highest { red_highest = count },
                        "green" => if count > green_highest { green_highest = count },
                        "blue" => if count > blue_highest { blue_highest = count },
                        _ => (),
                    }
                }
            }
            if red_highest <= max_red && green_highest <= max_green && blue_highest <= max_blue {
                result += game_id_number;
            }
        });
    result.to_string()
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
