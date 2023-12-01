fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let digits: String = line.chars().filter(|c| c.is_numeric()).collect();
        let mut number: String = String::new();

        number.push(digits.chars().nth(0).unwrap());
        number.push(digits.chars().rev().nth(0).unwrap());
        
        sum += number.parse::<i32>().unwrap();
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, 142);
    }
}