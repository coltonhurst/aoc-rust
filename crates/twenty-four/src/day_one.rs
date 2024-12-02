use std::collections::HashMap;

/*
    Given two lists, compare the
    smallest number in both the lists.
    Find the difference.
    Then, find the next smallest numbers,
    find the difference, etc.

    Sum the differences.
*/
pub fn part_one(input: Vec<String>) -> i32 {
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();
    let mut sum = 0;

    // Fill list_a and list_b, parsed from the input
    for line in input {
        let line_parts: Vec<&str> = line.split("   ").collect();
        list_a.push(line_parts.first().unwrap().parse().unwrap());
        list_b.push(line_parts.get(1).unwrap().parse().unwrap());
    }

    // Sort the lists in reverse order
    list_a.sort_by(|a, b| b.cmp(a));
    list_b.sort_by(|a, b| b.cmp(a));

    // Compare the difference between each index
    // Add the difference to sum
    while !list_a.is_empty() {
        sum += (list_a.pop().unwrap() - list_b.pop().unwrap()).abs();
    }

    sum
}

/*
    Given two lists, determine
    how many times each number
    from list a appears in list b.

    Multiply each number * the number of
    times it appears in list b.

    Sum these.
*/
pub fn part_two(input: Vec<String>) -> i32 {
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();
    let mut list_b_map: HashMap<i32, i32> = HashMap::new();
    let mut sum = 0;

    // Fill list_a and list_b, parsed from the input
    for line in input {
        let line_parts: Vec<&str> = line.split("   ").collect();
        list_a.push(line_parts.first().unwrap().parse().unwrap());
        list_b.push(line_parts.get(1).unwrap().parse().unwrap());
    }

    // Populate `list_b_map` with the numbers in list b with
    // the number of times they appear. This will be the "lookup"
    // table.
    for num in list_b {
        *list_b_map.entry(num).or_insert(0) += 1;

        // *** Old way ***
        // match list_b_map.get_key_value(&num) {
        //     Some(value) => _ = list_b_map.insert(num, value.1 + 1),
        //     None => _ = list_b_map.insert(num, 1),
        // }
    }

    // Multiply each number in list_a with the number of times
    // it appears using list_b_map. Sum these products.
    for num in list_a {
        if let Some((_, v)) = list_b_map.get_key_value(&num) {
            sum += num * v;
        }
    }

    sum
}
