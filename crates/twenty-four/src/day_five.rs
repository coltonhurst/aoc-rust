/*
    Given a set of rules and input,
    determine the correctly-ordered
    updates. Sum the middle page
    number.
*/
pub fn part_one(input: Vec<String>) -> i32 {
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    // Parse the rules & updates from the input
    let mut first_half = true;
    for line in input {
        if line.len() == 0 {
            first_half = false;
        }

        if first_half {
            let line_nums: Vec<i32> = line.split('|').map(|num| num.parse::<i32>().unwrap()).collect();
            rules.push((line_nums[0], line_nums[1]));
        } else {
            let line_nums: Vec<i32> = line.split(',').map(|num| num.parse::<i32>().unwrap()).collect();
            updates.push(line_nums);
        }
    }

    0
}

/*

*/
pub fn part_two(input: Vec<String>) -> i32 {
    0
}
