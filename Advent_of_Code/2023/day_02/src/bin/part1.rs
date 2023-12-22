fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

pub fn part1(input: &str) -> String {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let mut total = 0;
    let mut id = 0;
    let mut value = 0;

    let _output = input
        .lines()
        .map(|line|{
            for word in line.split_whitespace() {
                if word.contains(":") {
                    id = word[0..1].parse::<i32>().unwrap();
                } else if word.parse::<i32>().is_ok() {
                    value = word.parse::<i32>().unwrap();
                } else if word == "blue," {
                    if value > max_blue {
                        id = 0;
                        break;
                    }
                } else if word == "green," {
                    if value > max_green {
                        id = 0;
                        break;
                    }
                } else if word == "red," {
                    if value > max_red {
                        id = 0;
                        break;
                    }
                }
            }
            total = total + id;
        });
    total.to_string()
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
        assert_eq!(result, "8".to_string());
    }
}
