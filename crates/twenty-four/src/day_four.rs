/*
    Given a grid of input with consistent dimensions,
    find all occurrences of XMAS horizontally, vertically,
    and diagonally in both directions.

    Return the number of occurrences.

    Note: Input MUST be a square.
*/
pub fn part_one(input: Vec<String>) -> i32 {
    // The input as a grid of characters
    let mut grid: Vec<Vec<char>> = Vec::new();

    // Fill the grid from `input`
    for line in input {
        let mut new_line: Vec<char> = Vec::new();
        for character in line.chars() {
            new_line.push(character);
        }
        grid.push(new_line);
    }

    // Get 2d grids for horizontal checking
    // from the input vertically & horizontally
    let verticals = convert_verticals_into_horizontals(&grid);
    let diagonals_sw_to_ne = convert_diagonals_into_horizontals(&grid, DiagonalDirection::SWNE);
    let diagonals_se_to_nw = convert_diagonals_into_horizontals(&grid, DiagonalDirection::SENW);

    // Check occurrences of XMAS or SAMX and sum them
    check_grid_horizontally(&grid)
        + check_grid_horizontally(&verticals)
        + check_grid_horizontally(&diagonals_sw_to_ne)
        + check_grid_horizontally(&diagonals_se_to_nw)
}

/*
    Given a grid of input with consistent dimensions,
    find all occurrences of two"MAS's in the shape
    of an X.

    Example:

    M.S
    .A.
    M.S

    Return the number of occurrences.
*/
pub fn part_two(input: Vec<String>) -> i32 {
    // The input as a grid of characters
    let mut grid: Vec<Vec<char>> = Vec::new();

    // Fill the grid from `input`
    for line in input {
        let mut new_line: Vec<char> = Vec::new();
        for character in line.chars() {
            new_line.push(character);
        }
        grid.push(new_line);
    }

    // Loop through and find the X-MAS shape,
    // searching by the center 'A'
    let mut count = 0;
    for (x, _) in grid.iter().enumerate() {
        for (y, _) in grid[x].iter().enumerate() {
            // If we'll go out of bounds when checking,
            // or the character is not an 'A', skip this iteration
            if x < 1 || x >= grid.len() - 1 || y < 1 || y >= grid[x].len() - 1 || grid[x][y] != 'A'
            {
                continue;
            }

            // Create two lines through the center 'A'
            let swne_line: String = vec![grid[x + 1][y - 1], 'A', grid[x - 1][y + 1]]
                .iter()
                .collect();
            let senw_line: String = vec![grid[x + 1][y + 1], 'A', grid[x - 1][y - 1]]
                .iter()
                .collect();

            // If both lines have 'MAS' in either direction, increment count
            if (swne_line.contains("MAS") || reverse(&swne_line).contains("MAS"))
                && (senw_line.contains("MAS") || reverse(&senw_line).contains("MAS"))
            {
                count += 1;
            }
        }
    }

    count
}

enum DiagonalDirection {
    SWNE,
    SENW,
}

fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/*
    Given a grid of characters,
    return the number of times
    XMAS or SAMX appears horizontally.

    Example:
    X M A S
    S A M X
    X M A A
    A A A A

    Given the above input, the answer should be 2.

    NOTE: grid *will* be a jagged 2d vec at some point
*/
fn check_grid_horizontally(grid: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    for current_row in 0..grid.len() {
        // If there is no room in the row to search for XMAS
        // and SAMX, skip the row
        if grid[current_row].len() < 4 {
            continue;
        }

        // Search for XMAS and SAMX
        for current_column in 0..grid[current_row].len() {
            if current_column < grid[current_row].len() - 3 {
                if (grid[current_row][current_column] == 'X'
                    && grid[current_row][current_column + 1] == 'M'
                    && grid[current_row][current_column + 2] == 'A'
                    && grid[current_row][current_column + 3] == 'S')
                    || (grid[current_row][current_column] == 'S'
                        && grid[current_row][current_column + 1] == 'A'
                        && grid[current_row][current_column + 2] == 'M'
                        && grid[current_row][current_column + 3] == 'X')
                {
                    count += 1;
                }
            }
        }
    }

    count
}

/*
    Create a grid of vertical rows as horizontals.
    This assumes there are the same # of columns for each row.
*/
fn convert_verticals_into_horizontals(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for current_column in 0..input[0].len() {
        let mut line: Vec<char> = Vec::new();
        for current_row in 0..input.len() {
            line.push(input[current_row][current_column]);
        }
        result.push(line);
    }

    result
}

/*
    Given a 2d square vector of characters,
    turn the diagonals into horizontal lines.

    Diagonal direction can be:
    - SW->NE = DiagonalDirection::SWNE
    - SE->NW = DiagonalDirection::SENW

    -----

    Given the input, when converting the diagonals to
    a "jagged array" (or for rust, a jagged vec)...
    (going from SW -> NE)
    we should push the locations in the following
    order for each of the diagonals.

    # of diagonals: (square side length * 2) - 1

    Input:

    A B C D
    E F G H
    I J K L
    M N O P

    diagonal |  pushed locations in order
    --------------------------------------
        0    | (0, 0)
        1    | (1, 0) (0, 1)
        2    | (2, 0) (1, 1) (0, 2)
        3    | (3, 0) (2, 1) (1, 2) (0, 3)
        4    | (3, 1) (2, 2) (1, 3)
        5    | (3, 2) (2, 3)
        6    | (3, 3)

    Notice how the x values always decrease for each
    character & the y values increase. This is because
    of the SW -> NE direction.
*/
fn convert_diagonals_into_horizontals(
    input: &Vec<Vec<char>>,
    dir: DiagonalDirection,
) -> Vec<Vec<char>> {
    let mut result: Vec<Vec<char>> = Vec::new();

    let num_diagonals = input.len() * 2 - 1;
    let mut num_characters_in_line = 1;
    let mut starting_x = 0;
    let mut starting_y = 0;

    if let DiagonalDirection::SENW = dir {
        starting_y = input[0].len() - 1;
    }

    // Loop through every diagonal
    for current_diagonal in 0..num_diagonals {
        let mut line: Vec<char> = Vec::new();
        let mut x = starting_x;
        let mut y = starting_y;

        // Add each character in the line
        for _ in 0..num_characters_in_line {
            line.push(input[x][y]);

            if let DiagonalDirection::SENW = dir {
                if x > 0 && y > 0 {
                    x -= 1;
                    y -= 1;
                }
            } else if x > 0 {
                x -= 1;
                y += 1;
            }
        }

        // If dir = SENW
        if let DiagonalDirection::SENW = dir {
            // If we are in the first half of the diagonals
            if current_diagonal < num_diagonals / 2 {
                starting_x += 1;
                num_characters_in_line += 1;
            }
            // If we are at the middle diagonal or
            // second half of the diagonals
            else if current_diagonal >= num_diagonals / 2 && starting_y > 0 {
                starting_y -= 1;
                num_characters_in_line -= 1;
            }
        }
        // If dir = SWNE
        else {
            // If we are in the first half of the diagonals
            if current_diagonal < num_diagonals / 2 {
                starting_x += 1;
                num_characters_in_line += 1;
            }
            // If we are at the middle diagonal or
            // second half of the diagonals
            else if current_diagonal >= num_diagonals / 2 {
                starting_y += 1;
                num_characters_in_line -= 1;
            }
        }

        result.push(line);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_horizontal_success() {
        let input = vec![
            "AXMAS0".to_string(),
            "12789A".to_string(),
            "9SAMX0".to_string(),
            "13245L".to_string(),
            "XMAS13".to_string(),
            "M3A7S1".to_string(),
        ];

        assert_eq!(3, part_one(input));
    }

    #[test]
    fn part_one_vertical_success() {
        let input = vec![
            "AX0ASZ".to_string(),
            "S278XQ".to_string(),
            "A762MX".to_string(),
            "M324AR".to_string(),
            "XMA0SA".to_string(),
            "M3A7SO".to_string(),
        ];

        assert_eq!(2, part_one(input));
    }

    #[test]
    fn check_grid_horizontally_success() {
        let input: Vec<Vec<char>> = vec![
            vec!['X', 'A', 'S', '0'],
            vec!['S', 'A', 'M', 'X'],
            vec!['X', 'M', 'A', 'S'],
            vec!['0', 'M', 'A', 'S'],
        ];

        assert_eq!(2, check_grid_horizontally(&input));
    }

    #[test]
    fn convert_verticals_into_horizontals_success() {
        let input: Vec<Vec<char>> = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];

        let expected_output: Vec<Vec<char>> = vec![
            vec!['1', '4', '7'],
            vec!['2', '5', '8'],
            vec!['3', '6', '9'],
        ];

        assert_eq!(expected_output, convert_verticals_into_horizontals(&input));
    }

    #[test]
    fn convert_diagonals_into_horizontals_3x3_sw_ne_success() {
        let input: Vec<Vec<char>> = vec![
            vec!['1', '2', '3'],
            vec!['4', '5', '6'],
            vec!['7', '8', '9'],
        ];
        let expected_output: Vec<Vec<char>> = vec![
            vec!['1'],
            vec!['4', '2'],
            vec!['7', '5', '3'],
            vec!['8', '6'],
            vec!['9'],
        ];

        assert_eq!(
            expected_output,
            convert_diagonals_into_horizontals(&input, DiagonalDirection::SWNE)
        );
    }

    #[test]
    fn convert_diagonals_into_horizontals_4x4_sw_ne_success() {
        let input: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'D'],
            vec!['E', 'F', 'G', 'H'],
            vec!['I', 'J', 'K', 'L'],
            vec!['M', 'N', 'O', 'P'],
        ];
        let expected_output: Vec<Vec<char>> = vec![
            vec!['A'],
            vec!['E', 'B'],
            vec!['I', 'F', 'C'],
            vec!['M', 'J', 'G', 'D'],
            vec!['N', 'K', 'H'],
            vec!['O', 'L'],
            vec!['P'],
        ];

        assert_eq!(
            expected_output,
            convert_diagonals_into_horizontals(&input, DiagonalDirection::SWNE)
        );
    }

    #[test]
    fn convert_diagonals_into_horizontals_4x4_success() {
        let input: Vec<Vec<char>> = vec![
            vec!['A', 'B', 'C', 'D'],
            vec!['E', 'F', 'G', 'H'],
            vec!['I', 'J', 'K', 'L'],
            vec!['M', 'N', 'O', 'P'],
        ];
        let expected_output_swne: Vec<Vec<char>> = vec![
            vec!['A'],
            vec!['E', 'B'],
            vec!['I', 'F', 'C'],
            vec!['M', 'J', 'G', 'D'],
            vec!['N', 'K', 'H'],
            vec!['O', 'L'],
            vec!['P'],
        ];
        let expected_output_senw: Vec<Vec<char>> = vec![
            vec!['D'],
            vec!['H', 'C'],
            vec!['L', 'G', 'B'],
            vec!['P', 'K', 'F', 'A'],
            vec!['O', 'J', 'E'],
            vec!['N', 'I'],
            vec!['M'],
        ];

        assert_eq!(
            expected_output_swne,
            convert_diagonals_into_horizontals(&input, DiagonalDirection::SWNE)
        );
        assert_eq!(
            expected_output_senw,
            convert_diagonals_into_horizontals(&input, DiagonalDirection::SENW)
        );
    }

    #[test]
    fn part_two_success() {
        // let input: Vec<Vec<char>> = vec![
        //     vec!['.', 'M', '.', 'S', '.', '.', '.', '.', '.', '.'],
        //     vec!['.', '.', 'A', '.', '.', 'M', 'S', 'M', 'S', '.'],
        //     vec!['.', 'M', '.', 'S', '.', 'M', 'A', 'A', '.', '.'],
        //     vec!['.', '.', 'A', '.', 'A', 'S', 'M', 'S', 'M', '.'],
        //     vec!['.', 'M', '.', 'S', '.', 'M', '.', '.', '.', '.'],
        //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        //     vec!['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', '.'],
        //     vec!['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', '.'],
        //     vec!['M', '.', 'M', '.', 'M', '.', 'M', '.', 'M', '.'],
        //     vec!['.', '.', '.', '.', '.', '.', '.', '.', '.', '.'],
        // ];

        let input = vec![
            ".M.S......".to_string(),
            "..A..MSMS.".to_string(),
            ".M.S.MAA..".to_string(),
            "..A.ASMSM.".to_string(),
            ".M.S.M....".to_string(),
            "..........".to_string(),
            "S.S.S.S.S.".to_string(),
            ".A.A.A.A..".to_string(),
            "M.M.M.M.M.".to_string(),
            "..........".to_string(),
        ];

        assert_eq!(9, part_two(input));
    }
}
