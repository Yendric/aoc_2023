fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn string_to_char_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_adjacent(col1: usize, row1: usize, col2: usize, row2: usize) -> bool {
    let col_diff = col1 as isize - col2 as isize;
    let row_diff = row1 as isize - row2 as isize;

    col_diff.abs() <= 1 && row_diff.abs() <= 1
}

// I'm so sorry
fn get_gear_product(matrix: &Vec<Vec<char>>, gear_row: usize, gear_col: usize) -> i32 {
    let mut numbers: Vec<i32> = Vec::new();

    let mut current_number = String::new();
    // Check left of gear
    for i in (0..gear_col).rev() {
        let char = matrix[gear_row][i as usize];
        if char.is_digit(10) {
            current_number.insert(0, char);
        } else {
            break;
        }
    }
    if current_number.len() > 0 {
        numbers.push(current_number.parse::<i32>().unwrap());
    }
    current_number.clear();

    // Check right of gear
    if gear_col + 1 < matrix[0].len() {
        for i in (gear_col + 1)..matrix[0].len() {
            let char = matrix[gear_row][i];
            if char.is_digit(10) {
                current_number.push(char);
            } else {
                break;
            }
        }
        if current_number.len() > 0 {
            numbers.push(current_number.parse::<i32>().unwrap());
        }
        current_number.clear();
    }

    // Check above gear
    if gear_row as i32 - 1 >= 0 {
        let mut adjacent = false;
        for i in 0..matrix[0].len() {
            let char = matrix[gear_row - 1][i];
            if char.is_digit(10) {
                current_number.push(char);

                if is_adjacent(gear_col, gear_row, i, gear_row - 1) {
                    adjacent = true
                }
            }
            if !char.is_digit(10) || i == matrix[0].len() - 1 {
                if adjacent {
                    numbers.push(current_number.parse::<i32>().unwrap());
                }

                current_number.clear();
                adjacent = false;
            }
        }
    }
    current_number.clear();

    // Check below gear
    if gear_row + 1 < matrix.len() {
        let mut adjacent = false;
        for i in 0..matrix[0].len() {
            let char = matrix[gear_row + 1][i];
            if char.is_digit(10) {
                current_number.push(char);

                if is_adjacent(gear_col, gear_row, i, gear_row + 1) {
                    adjacent = true
                }
            }
            if !char.is_digit(10) || i == matrix[0].len() - 1 {
                if adjacent {
                    numbers.push(current_number.parse::<i32>().unwrap());
                }

                current_number.clear();
                adjacent = false;
            }
        }
    }
    current_number.clear();

    if numbers.len() == 2 {
        println!("{:?}", numbers);
        return numbers[0] * numbers[1];
    }

    return 0;
}

fn part2(input: &str) -> i32 {
    let mut sum = 0;
    let matrix = string_to_char_matrix(input);

    for (row_index, row) in matrix.iter().enumerate() {
        for (col_index, &char) in row.iter().enumerate() {
            if char == '*' {
                sum += get_gear_product(&matrix, row_index, col_index);
            }
        }
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 467835);
    }
}
