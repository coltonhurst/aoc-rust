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
    'outer: for level in numbers {
        let mut ascending = true;

        // Iterate through each number in the level
        'inner: for (index, num) in level.iter().enumerate() {
            if index == 0 {
                // Skip the check for the first number
                continue 'inner;
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
                // If any of this fails, break out of the inner loop,
                // because this level is invalid.
                let last_num = level.get(index - 1).unwrap();
                let diff = (num - last_num).abs();
                if !(1..=3).contains(&diff)
                    || ascending && num < last_num
                    || !ascending && num > last_num
                {
                    continue 'outer;
                }
            }

            // If we are on the last number and it has passed all the checks,
            // increment `num_valid_lines`, as the line is valid.
            if index == level.len() - 1 {
                num_valid_lines += 1;
            }
        }
    }

    num_valid_lines
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
}
