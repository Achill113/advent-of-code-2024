use tracing::instrument;

#[instrument]
pub fn process(_input: &str) -> miette::Result<String> {
    let mut result = 0;
    let letter_matrix: Vec<Vec<char>> = _input.lines().map(|line| line.chars().collect()).collect();
    let row_count = letter_matrix.len();

    for (y, line) in letter_matrix.clone().into_iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'A' {
                let current_position: [usize; 2] = [x, y];

                // check 4 corners. if opposite corners are M and S on both sides, we found x-mas
                if let Some(corners) = get_corners(&letter_matrix, current_position, [line.len(), row_count]) {
                    if corners[0].contains(&'M') && corners[0].contains(&'S') && corners[1].contains(&'M') && corners[1].contains(&'S') {
                        result += 1;
                    }
                }
            }
        }
    }

    Ok(result.to_string())
}

// output corner pairs like [[top-left, bottom-right], [top-right, bottom-left]]
fn get_corners(
    letter_matrix: &Vec<Vec<char>>,
    position: [usize; 2],
    matrix_size: [usize; 2],
) -> Option<[[char; 2]; 2]> {
    let position_x = position[0];
    let position_y = position[1];

    if position_x == 0
        || position_x == matrix_size[0] - 1
        || position_y == 0
        || position_y == matrix_size[1] - 1
    {
        return None;
    }

    let x_range = position_x - 1..=position_x + 1;
    let y_range = position_y - 1..=position_y + 1;

    let mut top_left: char = 'Z';
    let mut top_right: char = 'Z';
    let mut bottom_left: char = 'Z';
    let mut bottom_right: char = 'Z';

    for y in y_range {
        for x in x_range.clone() {
            let direction_x = (x as isize - position_x as isize).signum();
            let direction_y = (y as isize - position_y as isize).signum();

            if direction_x == -1 && direction_y == 1 {
                top_left = letter_matrix[y][x];
            }

            if direction_x == 1 && direction_y == 1 {
                top_right = letter_matrix[y][x];
            }

            if direction_x == -1 && direction_y == -1 {
                bottom_left = letter_matrix[y][x];
            }

            if direction_x == 1 && direction_y == -1 {
                bottom_right = letter_matrix[y][x];
            }
        }
    }

    return Some([[top_left, bottom_right], [top_right, bottom_left]]);
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
        assert_eq!("9", process(input)?);
        Ok(())
    }
}
