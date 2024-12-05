use std::cmp;

const TARGET: &str = "XMAS";

#[tracing::instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result = 0;
    let letter_matrix: Vec<Vec<char>> = _input.lines().map(|line| line.chars().collect()).collect();
    let row_count = letter_matrix.len();

    for (y, line) in letter_matrix.clone().into_iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            print!("{char}");
            if char == &'X' {
                println!("\nFound X at {x}, {y}");
                let mut current_position: [usize; 2] = [x, y];
                let mut char_index = 1;
                let mut target_char = TARGET.chars().nth(char_index).unwrap();

                if let Some(direction) = check_neighbors(
                    &letter_matrix,
                    current_position,
                    [line.len(), row_count],
                    &target_char,
                ) {
                    let new_position = [
                        current_position[0] as isize + (direction[0] * 2),
                        current_position[1] as isize + (direction[1] * 2),
                    ];
                    if new_position[0] >= 0
                        && new_position[1] >= 0
                        && new_position[0] < line.len() as isize
                        && new_position[1] < row_count as isize
                    {
                        current_position[0] = new_position[0] as usize;
                        current_position[1] = new_position[1] as usize;

                        char_index += 1;
                        target_char = TARGET.chars().nth(char_index).unwrap();

                        while check_char_at_position(&letter_matrix, current_position, &target_char)
                        {
                            if char_index == TARGET.len() - 1 {
                                result += 1;
                                break;
                            }
                            let new_position = [
                                current_position[0] as isize + direction[0],
                                current_position[1] as isize + direction[1],
                            ];

                            if new_position[0] < 0
                                || new_position[1] < 0
                                || new_position[0] >= line.len() as isize
                                || new_position[1] >= row_count as isize
                            {
                                break;
                            }

                            current_position[0] =
                                (current_position[0] as isize + direction[0]) as usize;
                            current_position[1] =
                                (current_position[1] as isize + direction[1]) as usize;

                            char_index += 1;
                            target_char = TARGET.chars().nth(char_index).unwrap();
                        }
                    }
                }
            }
        }
    }

    Ok(result.to_string())
}

fn check_neighbors(
    letter_matrix: &Vec<Vec<char>>,
    position: [usize; 2],
    matrix_size: [usize; 2],
    target_char: &char,
) -> Option<[isize; 2]> {
    let position_x = position[0];
    let position_y = position[1];

    let x_range = if position_x == 0 {
        0..=1
    } else {
        position_x - 1..=cmp::min(position_x + 1, matrix_size[0] - 1)
    };
    let y_range = if position_y == 0 {
        0..=1
    } else {
        position_y - 1..=cmp::min(position_y + 1, matrix_size[1] - 1)
    };

    for y in y_range {
        for x in x_range.clone() {
            if check_char_at_position(letter_matrix, [x, y], target_char) {
                let direction_x = (x as isize - position_x as isize).signum();
                let direction_y = (y as isize - position_y as isize).signum();

                return Some([direction_x, direction_y]);
            }
        }
    }

    None
}

fn check_char_at_position(
    letter_matrix: &Vec<Vec<char>>,
    position: [usize; 2],
    char: &char,
) -> bool {
    if &letter_matrix[position[1]][position[0]] == char {
        println!("Found {char} at {}, {}", position[0], position[1]);
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
