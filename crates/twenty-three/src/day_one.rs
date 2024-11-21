const BASE_TEN: u32 = 10;

/*
    For each line of input, combine the first
    number character and last number character
    to form a two digit number. Sum these numbers
    from each line, and that's the solution.
*/
pub fn part_one(input: Vec<String>) -> u32 {
    let mut sum = 0;

    for input_line in input {
        let mut left_num: Option<char> = None;
        let mut right_num: Option<char> = None;

        for character in input_line.chars() {
            if character.is_digit(BASE_TEN) {
                if left_num.is_none() {
                    left_num = Some(character);
                } else {
                    right_num = Some(character);
                }
            }

            if right_num.is_none() {
                right_num = left_num;
            }
        }

        if let (Some(left_num), Some(right_num)) = (left_num, right_num) {
            if let Ok(num) = format!("{}{}", left_num, right_num).parse::<u32>() {
                sum += num;
            }
        }
    }

    sum
}

/*
    For each line of input, combine the first
    number character and last number character
    to form a two digit number. This could also be
    a number spelled out as a word. Sum these numbers
    from each line, and that's the solution.
*/
pub fn part_two(input: Vec<String>) -> u32 {
    let mut sum = 0;

    for input_line in input {
        let numbers_in_line = get_numbers_in_line(input_line);

        let number = if numbers_in_line.is_empty() {
            String::from("0")
        } else if numbers_in_line.len() == 1 {
            format!("{}{}", numbers_in_line[0], numbers_in_line[0])
        } else {
            format!(
                "{}{}",
                numbers_in_line[0],
                numbers_in_line[numbers_in_line.len() - 1]
            )
        };

        if let Ok(num) = number.parse::<u32>() {
            sum += num;
        }
    }

    sum
}

/*
    Given a string, return the numbers in that
    line in order. Includes words of numbers.
*/
fn get_numbers_in_line(line: String) -> Vec<u32> {
    let line = line.to_lowercase();
    let mut numbers_in_line: Vec<u32> = Vec::new();

    for (index, character) in line.chars().enumerate() {
        if let Some(digit) = character.to_digit(BASE_TEN) {
            numbers_in_line.push(digit);
        } else if (index + 3 <= line.len()) && &line[index..index + 3] == "one" {
            numbers_in_line.push(1);
        } else if (index + 3 <= line.len()) && &line[index..index + 3] == "two" {
            numbers_in_line.push(2);
        } else if (index + 5 <= line.len()) && &line[index..index + 5] == "three" {
            numbers_in_line.push(3);
        } else if (index + 4 <= line.len()) && &line[index..index + 4] == "four" {
            numbers_in_line.push(4);
        } else if (index + 4 <= line.len()) && &line[index..index + 4] == "five" {
            numbers_in_line.push(5);
        } else if (index + 3 <= line.len()) && &line[index..index + 3] == "six" {
            numbers_in_line.push(6);
        } else if (index + 5 <= line.len()) && &line[index..index + 5] == "seven" {
            numbers_in_line.push(7);
        } else if (index + 5 <= line.len()) && &line[index..index + 5] == "eight" {
            numbers_in_line.push(8);
        } else if (index + 4 <= line.len()) && &line[index..index + 4] == "nine" {
            numbers_in_line.push(9);
        }
    }

    numbers_in_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_success() {
        let input = vec!["ab2c3".to_string(), "9a9".to_string(), "b8b".to_string()];

        assert_eq!(210, part_one(input));
    }

    #[test]
    fn part_two_success() {
        let input = vec![
            "ab2c3".to_string(),
            "9anine".to_string(),
            "beightb".to_string(),
        ];

        assert_eq!(210, part_two(input));
    }

    #[test]
    fn get_numbers_in_line_success() {
        let line = String::from("t8one9twoPthreeQfourfive");

        assert_eq!(vec![8, 1, 9, 2, 3, 4, 5], get_numbers_in_line(line));
    }
}
