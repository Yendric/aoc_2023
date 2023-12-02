fn main() {
    let input = include_str!("./input2.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut power_sum = 0;

    for game in input.lines() {
        let mut game_part = game.split(": ");

        power_sum += game_min_power(game_part.nth(1).unwrap())
    }

    return power_sum;
}

fn game_min_power(game: &str) -> i32 {
    let sets = game.split("; ");

    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;

    for set in sets {
        let per_color = set.split(", ");

        for color_grab in per_color {
            let mut amount_color = color_grab.split(" ");
            let amount: i32 = amount_color.nth(0).unwrap().parse().unwrap();
            let color = amount_color.nth(0).unwrap();

            if amount > min_red && color == "red" {
                min_red = amount;
            } else if amount > min_blue && color == "blue" {
                min_blue = amount;
            } else if amount > min_green && color == "green" {
                min_green = amount;
            }
        }
    }

    return min_red * min_blue * min_green;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 2286);
    }
}
