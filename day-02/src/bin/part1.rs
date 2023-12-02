const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut id_sum = 0;

    for game in input.lines() {
        let mut game_part = game.split(": ");
        let game_id = game_part.nth(0).unwrap().replace("Game ", "");

        if is_game_valid(game_part.nth(0).unwrap()) {
            id_sum += game_id.parse::<i32>().unwrap();
        }
    }

    return id_sum;
}

fn is_game_valid(game: &str) -> bool {
    let sets = game.split("; ");

    for set in sets {
        let per_color = set.split(", ");

        for color_grab in per_color {
            let mut amount_color = color_grab.split(" ");
            let amount: i32 = amount_color.nth(0).unwrap().parse().unwrap();
            let color = amount_color.nth(0).unwrap();

            if amount > MAX_RED && color == "red"
                || amount > MAX_BLUE && color == "blue"
                || amount > MAX_GREEN && color == "green"
            {
                return false;
            }
        }
    }

    return true;
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
        assert_eq!(result, 8);
    }
}
