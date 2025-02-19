fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

pub fn part1(input: &str) -> String {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut result = 0;

    let _output = input
        .lines()
        .map(|line|{
            let mut game = line.split(": ");
            println!("{:?}", game);
            let _game_id = game.next().unwrap();
            println!("{:?}", _game_id);
            let game = game.next().unwrap();
            let colors = game.split("; ");
            println!("{:?}", colors);
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for color in colors {
                let color = color.split(", ");
                for c in color {
                    let mut c = c.split_whitespace();
                    let count = c.next().unwrap().parse::<i32>().unwrap();
                    let color = c.next().unwrap();
                    match color {
                        "red" => red += count,
                        "green" => green += count,
                        "blue" => blue += count,
                        _ => (),
                    }
                }
            }
            if red <= max_red && green <= max_green && blue <= max_blue {
                result += _game_id.parse::<i32>().unwrap();
            }
        });
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        let answer = 1 + 2 + 5; // Game 1, Game 2, and Game 5 are valid 
        assert_eq!(result, answer.to_string());
    }
}
