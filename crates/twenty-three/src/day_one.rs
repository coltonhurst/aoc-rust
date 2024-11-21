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
            if let Some(_) = character.to_digit(BASE_TEN) {
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
