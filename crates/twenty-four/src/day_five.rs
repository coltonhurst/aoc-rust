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
            let line_nums: Vec<i32> = line
                .split('|')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            rules.push((line_nums[0], line_nums[1]));
        } else {
            let line_nums: Vec<i32> = line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            updates.push(line_nums);
        }
    }

    // Add the midpoint to sum for valid updates
    let mut sum = 0;
    for update in updates {
        if update_is_valid(&update, &rules) {
            sum += update[update.len() / 2];
        }
    }

    sum
}

fn update_is_valid(update: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    // Check to make sure each number follows all rules
    for (number_index, number) in update.iter().enumerate() {
        // Loop through every rule to check
        for rule in rules {
            println!("something");
        }
    }

    false
}

/*

*/
pub fn part_two(input: Vec<String>) -> i32 {
    0
}
