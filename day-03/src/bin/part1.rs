fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn string_to_char_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn is_symbol(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let char = matrix[row][col];

    return (!char.is_digit(10)) && char != '.';
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    let matrix = string_to_char_matrix(input);

    for (row_index, row) in matrix.iter().enumerate() {
        let mut current_number = String::new();
        let mut is_adjacent = false;

        for (col_index, &char) in row.iter().enumerate() {
            if char.is_digit(10) {
                current_number.push(char);
                // Check TBLRD
                if (row_index as i32 - 1 >= 0 && is_symbol(&matrix, row_index - 1, col_index))
                    || (row_index + 1 < matrix.len() && is_symbol(&matrix, row_index + 1, col_index))
                    || (col_index as i32 - 1 >= 0 && is_symbol(&matrix, row_index, col_index - 1))
                    || (col_index + 1 < row.len() && is_symbol(&matrix, row_index, col_index + 1))
                    || (row_index + 1 < matrix.len() && col_index + 1 < row.len() && is_symbol(&matrix, row_index + 1, col_index + 1)) // BOTTOM RIGHT
                    || (row_index + 1 < matrix.len() && col_index as i32 - 1 >= 0 && is_symbol(&matrix, row_index + 1, col_index - 1)) // BOTTOM LEFT 
                    || (row_index as i32 - 1 >= 0 && col_index + 1 < row.len() && is_symbol(&matrix, row_index - 1, col_index + 1)) // TOP RIGHT
                    || (row_index as i32 - 1 >= 0 && col_index as i32 - 1 >= 0 && is_symbol(&matrix, row_index - 1, col_index - 1))
                // TOP LEFT
                {
                    is_adjacent = true;
                }
            }
            if !char.is_digit(10) || col_index == row.len() - 1 {
                if is_adjacent {
                    sum += current_number.parse::<i32>().unwrap();
                }

                current_number.clear();
                is_adjacent = false;
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
        let result = part1(
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
        assert_eq!(result, 4361);
    }
}
