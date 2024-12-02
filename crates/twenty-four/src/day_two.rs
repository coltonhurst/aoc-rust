/*
    Given a list of numbers, each line
    is valid if the whole line is either
    increasing OR decreasing, and the difference
    between the numbers is at least 1 and at
    most 3.

    Return the number of valid lines.
*/
pub fn part_one(input: Vec<String>) -> i32 {
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    let mut num_valid_lines = 0;

    // Fill the 2d `numbers` vector with parsed input
    for line in input {
        let line_numbers: Vec<&str> = line.split(" ").collect();
        let mut levels: Vec<i32> = Vec::new();

        // For each number in the line, parse it,
        // and add it to `levels`
        for num in line_numbers {
            levels.push(num.parse().unwrap());
        }

        // Add `levels` to `numbers`
        numbers.push(levels);
    }

    // Check if each level in `numbers` is valid
    // If it is, increment `num_valid_lines`
    for level in numbers {
        if level_is_valid(level) {
            num_valid_lines += 1;
        }
    }

    num_valid_lines
}

/*
    Given a list of numbers, each line
    is valid if the whole line is either
    increasing OR decreasing, and the difference
    between the numbers is at least 1 and at
    most 3.

    Return the number of valid lines.

    This is the same as part one, with one change:
    If removing a single level from an unsafe report
    makes it safe, then the report can be considered
    safe.
*/
pub fn part_two(input: Vec<String>) -> i32 {
    let mut numbers: Vec<Vec<i32>> = Vec::new();
    let mut num_valid_lines = 0;

    // Fill the 2d `numbers` vector with parsed input
    for line in input {
        let line_numbers: Vec<&str> = line.split(" ").collect();
        let mut levels: Vec<i32> = Vec::new();

        // For each number in the line, parse it,
        // and add it to `levels`
        for num in line_numbers {
            levels.push(num.parse().unwrap());
        }

        // Add `levels` to `numbers`
        numbers.push(levels);
    }

    // Check if each level in `numbers` is valid
    // If it is, increment `num_valid_lines`.
    // If not, try to remove one item to see
    // if that will make the level pass. If so,
    // increment `num_valid_lines`.
    for level in numbers {
        if level_is_valid(level.clone()) {
            num_valid_lines += 1;
        } else {
            let mut count = 0;

            // Loop through and see if
            // removing one of the numbers
            // from the level allows it to pass.
            // If so, inccrement `num_valid_lines`.
            while count < level.len() {
                let mut altered_level = level.clone();
                _ = altered_level.remove(count);

                if level_is_valid(altered_level) {
                    num_valid_lines += 1;
                    break;
                }
                count += 1;
            }
        }
    }

    num_valid_lines
}

// Given a level, determine if the level is valid
// A level is valid if:
//   - All numbers are ascending or descending
//   - The difference between numbers is min 1, max 3
fn level_is_valid(level: Vec<i32>) -> bool {
    let mut ascending = true;

    // Iterate through each number in the level
    for (index, num) in level.iter().enumerate() {
        if index == 0 {
            // Skip the check for the first number
            continue;
        } else if index == 1 {
            // Set index to determine if the level is consistently
            // ascending or descending
            //   - `ascending` should be true is index 1 > index 0
            //   - `ascending` should be false if index 1 < index 0
            ascending = num > level.first().unwrap();
        }

        if index > 0 {
            // Verify:
            //   - The current number follows the pattern established
            //     by `ascending`
            //   - There is a difference of at least 1 and at max 3
            // If any of this fails, return false,
            // because this level is invalid.
            let last_num = level.get(index - 1).unwrap();
            let diff = (num - last_num).abs();
            if !(1..=3).contains(&diff)
                || ascending && num < last_num
                || !ascending && num > last_num
            {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_success() {
        let input = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];

        assert_eq!(2, part_one(input));
    }

    #[test]
    fn part_two_success() {
        let input = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];

        assert_eq!(4, part_two(input));
    }

    #[test]
    fn level_is_valid_success() {
        assert!(level_is_valid(vec![7, 6, 4, 2, 1]));
        assert!(level_is_valid(vec![1, 3, 6, 7, 9]));
    }

    #[test]
    fn level_is_valid_failure() {
        assert!(!level_is_valid(vec![1, 2, 7, 8, 9]));
        assert!(!level_is_valid(vec![9, 7, 6, 2, 1]));
        assert!(!level_is_valid(vec![1, 3, 2, 4, 5]));
        assert!(!level_is_valid(vec![8, 6, 4, 4, 1]));
    }
}
