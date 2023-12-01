use std::char::from_digit;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum = 0;

    for line in input.lines() {

        let mut first_number: Option<char> = None;
        let mut last_number: Option<char> = None;

        for word_iterator in 0..line.len() {
            let current_char = line.chars().nth(word_iterator).unwrap();
            let mut digit: Option<u32> = None;

            if current_char.is_digit(10) {
                digit = Some(current_char.to_digit(10).unwrap());
            } else {
                for char_iterator in word_iterator..line.len() {
                    let current_word = &line[word_iterator..=char_iterator];

                    if numbers.contains(&current_word) {
                        let position:u32 = numbers.iter().position(|&c| c == current_word).unwrap() as u32;        
                        digit = Some(position + 1);
                    }
                }
            }
            
            match digit {
                Some(x) =>  {
                    if first_number.is_none() && !digit.is_none() {
                        first_number = Some(from_digit(x, 10).unwrap());
                    } 
                    if !digit.is_none()  {
                        last_number = Some(from_digit(x, 10).unwrap());
                    }
                },
                None => {}
            };
        }

        let mut number: String = String::new();
        match first_number {
            Some(x) => {
                number.push(x);      
            },
            None => {}
        }

        match last_number {
            Some(x) => {
                number.push(x)
            },
            None => {}
        }

        println!("{}", number);
         
        sum += number.parse::<i32>().unwrap();   
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
sevenine");
        assert_eq!(result, 281);
    }
}