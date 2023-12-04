fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> usize {
    let mut card_counts = vec![0; input.lines().count()];

    for (index, line) in input.lines().enumerate() {
        let mut line_sum = 0;
        let numbers = line.split(": ").nth(1).unwrap();
        let part1 = numbers.split(" | ").nth(0).unwrap();
        let part2 = numbers.split(" | ").nth(1).unwrap();

        let winning_numbers = part1.split(' ').filter(|&x| !x.parse::<i32>().is_err());
        let my_numbers = part2.split(' ').filter(|&x| !x.parse::<i32>().is_err());

        for number in my_numbers {
            if winning_numbers.clone().any(|x| x == number) {
                line_sum += 1;
            }
        }

        card_counts[index] += 1;

        for i in 1..=line_sum {
            if index + i < card_counts.len() {
                card_counts[index + i] += card_counts[index];
            }
        }
    }

    return card_counts.into_iter().sum::<usize>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 30);
    }
}
