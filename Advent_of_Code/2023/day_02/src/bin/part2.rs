fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

pub fn part2(_input: &str) -> String {
    let mut result = 0;

    let _output = _input
        .lines()
        .for_each(|line|{
            let mut game = line.split(": ");
            let _game_id = game.next().unwrap();
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
            result += red_highest*green_highest*blue_highest;
        });
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "2286".to_string());
    }
}
