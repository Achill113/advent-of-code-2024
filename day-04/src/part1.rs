use std::cmp;

use tracing::instrument;

const TARGET: &str = "XMAS";

#[instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result = 0;
    let letter_matrix: Vec<Vec<char>> = _input.lines().map(|line| line.chars().collect()).collect();
    let row_count = letter_matrix.len();

    for (y, line) in letter_matrix.clone().into_iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'X' {
                let current_position: [usize; 2] = [x, y];
                let mut char_index = 1;
                let mut target_char = TARGET.chars().nth(char_index).unwrap();

                if let Some(directions) = check_neighbors(
                    &letter_matrix,
                    current_position,
                    [line.len(), row_count],
                    &target_char,
                ) {
                    // loop through all Ms surrounding the current X
                    for direction in directions {
                        // reset back to the current_position for each loop
                        let mut inner_position = current_position.clone();

                        let new_position = [
                            inner_position[0] as isize + (direction[0] * 2), // multiply by 2 because we need to skip past the M we already found and check if the next char in this direction is A
                            inner_position[1] as isize + (direction[1] * 2),
                        ];

                        // check that we're still in bounds
                        if new_position[0] >= 0
                            && new_position[1] >= 0
                            && new_position[0] < line.len() as isize
                            && new_position[1] < row_count as isize
                        {
                            inner_position[0] = new_position[0] as usize;
                            inner_position[1] = new_position[1] as usize;

                            // we've found X and M, so we need to look for A (char_index 2)
                            char_index = 2;
                            target_char = TARGET.chars().nth(char_index).unwrap();

                            while check_char_at_position(
                                &letter_matrix,
                                inner_position,
                                &target_char,
                            ) {
                                // we matched an S, we found XMAS!
                                if char_index == TARGET.len() - 1 {
                                    result += 1;
                                    break;
                                }

                                // move to the next position in the current direction
                                let new_position = [
                                    inner_position[0] as isize + direction[0],
                                    inner_position[1] as isize + direction[1],
                                ];

                                // if we're out of bounds now, break out
                                if new_position[0] < 0
                                    || new_position[1] < 0
                                    || new_position[0] >= line.len() as isize
                                    || new_position[1] >= row_count as isize
                                {
                                    break;
                                }

                                inner_position[0] = new_position[0] as usize; // we've already checked that its not a negative number
                                inner_position[1] = new_position[1] as usize;

                                char_index += 1;
                                target_char = TARGET.chars().nth(char_index).unwrap();
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(result.to_string())
}

#[instrument]
fn check_neighbors(
    letter_matrix: &Vec<Vec<char>>,
    position: [usize; 2],
    matrix_size: [usize; 2],
    target_char: &char,
) -> Option<Vec<[isize; 2]>> {
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

    let mut matches: Vec<[isize; 2]> = vec![];

    for y in y_range {
        for x in x_range.clone() {
            if check_char_at_position(letter_matrix, [x, y], target_char) {
                let direction_x = (x as isize - position_x as isize).signum();
                let direction_y = (y as isize - position_y as isize).signum();

                matches.push([direction_x, direction_y]);
            }
        }
    }

    if matches.len() > 0 {
        return Some(matches);
    } else {
        return None;
    }
}

#[instrument]
fn check_char_at_position(
    letter_matrix: &Vec<Vec<char>>,
    position: [usize; 2],
    char: &char,
) -> bool {
    &letter_matrix[position[1]][position[0]] == char
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
