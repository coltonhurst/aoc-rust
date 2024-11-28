/*
    Only one line of input, sum:

    ( +1
    ) -1
*/
pub fn part_one(input: Vec<String>) -> i32 {
    let mut sum = 0;

    for parenthesis in input[0].chars() {
        match parenthesis {
            '(' => sum = sum + 1,
            ')' => sum = sum - 1,
            _ => (),
        }
    }

    sum
}

/*
    Only one line of input, sum:

    ( +1
    ) -1

    When Santa enters the basement,
    return the location of the parenthesis
    that sent him there (floor = -1).

    Note: starting char is position 1
*/
pub fn part_two(input: Vec<String>) -> usize {
    let mut sum = 0;

    for (index, parenthesis) in input[0].chars().enumerate() {
        match parenthesis {
            '(' => sum = sum + 1,
            ')' => sum = sum - 1,
            _ => (),
        }

        if sum == -1 {
            return index + 1; // position 0 is 1
        }
    }

    0
}
