use std::collections::HashMap;

/*
    Given a set of rules and input,
    determine the correctly-ordered
    updates. Sum the middle page
    number.
*/
pub fn part_one(input: Vec<String>) -> i32 {
    let (rules, updates) = parse_input(input);

    // Sum the midpoint of correctly ordered updates
    let mut sum = 0;
    for update in updates {
        if update_is_valid(&update, &rules) {
            sum += update[update.len() / 2];
        }
    }

    sum
}

/*
    Given a set of rules and input,
    determine the incorrectly-ordered
    updates. Order them based on the rules,
    and sum the middle page number.
*/
pub fn part_two(input: Vec<String>) -> i32 {
    // COPIED PART 1, NOT STARTED YET

    let (rules, updates) = parse_input(input);

    // Sum the midpoint of correctly ordered updates
    let mut sum = 0;
    for update in updates {
        if update_is_valid(&update, &rules) {
            sum += update[update.len() / 2];
        }
    }

    sum
}

/*
    Given the day 5 input as a Vec<String>,
    return the rules and update lines.
*/
pub fn parse_input(input: Vec<String>) -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    // Parse the rules & updates from the input
    let mut first_half = true;
    for line in input {
        if line.len() == 0 {
            first_half = false;
        } else if first_half {
            let line_nums: Vec<i32> = line
                .split('|')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();

            if rules.contains_key(&line_nums[0]) {
                rules.get_mut(&line_nums[0]).unwrap().push(line_nums[1]);
            } else {
                rules.insert(line_nums[0], vec![line_nums[1]]);
            }
        } else {
            let line_nums: Vec<i32> = line
                .split(',')
                .map(|num| num.parse::<i32>().unwrap())
                .collect();

            updates.push(line_nums);
        }
    }

    (rules, updates)
}

/*
    Given a vec of numbers,
    return true if they follow the rules.

    Ex of a valid update:
      Update: [1, 7, 3, 9, 5]
        Rule: 3 | (4, 9)
        (means 3 must appear before 4 and 9 in the vec)
*/
fn update_is_valid(update: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> bool {
    // Check to make sure each number follows all rules
    for (number_index, number) in update.iter().enumerate() {
        // Skip the first number
        if number_index == 0 {
            continue;
        }

        // Create a vec of the items to the left
        let left_of_num = update[0..number_index].to_vec();

        // Create a vec of the applicable rule numbers that should be to
        // the right of the number
        let mut applicable_ruleset = rules[number].clone();

        // If any number from the rule set is to the left,
        // return false, as it's invalid
        applicable_ruleset.retain(|n| left_of_num.contains(n));
        if applicable_ruleset.len() > 0 {
            return false;
        }
    }

    true
}
