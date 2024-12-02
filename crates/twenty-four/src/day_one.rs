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
        list_a.push(line_parts.get(0).unwrap().parse().unwrap());
        list_b.push(line_parts.get(1).unwrap().parse().unwrap());
    }

    // Sort the lists in reverse order
    list_a.sort_by(|a, b| b.cmp(a));
    list_b.sort_by(|a, b| b.cmp(a));

    // Compare the difference between each index
    // Add the difference to sum
    while list_a.len() > 0 {
        sum += (list_a.pop().unwrap() - list_b.pop().unwrap()).abs();
    }

    sum
}
