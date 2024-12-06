/*
    Given a square map with movement
    rules for the guard, find
    how many unique locations
    the guard will visit on the map.

    Map: (height, width)

     0,0 ----> (0, width_max)
     |
     |
     |
     V
    (height_max, 0)
*/
pub fn part_one(input: Vec<String>) -> i32 {
    let mut map: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
    let height_max: i32 = map.len() as i32 - 1;
    let width_max: i32 = map[0].len() as i32 - 1;
    let mut guard_loc = find_guard_starting_location(&map);
    let mut guard_outside_map = false;

    // Move the guard until the guard exits the map
    while !guard_outside_map {
        let mut next_guard_loc = (guard_loc.0 as i32, guard_loc.1 as i32);

        next_guard_loc = match map[guard_loc.0][guard_loc.1] {
            '^' => (next_guard_loc.0 - 1, next_guard_loc.1),
            '>' => (next_guard_loc.0, next_guard_loc.1 + 1),
            'v' => (next_guard_loc.0 + 1, next_guard_loc.1),
            '<' => (next_guard_loc.0, next_guard_loc.1 - 1),
            _ => panic!("can't determine next guard location"),
        };

        // If the next location is outside the map bounds
        if next_guard_loc.0 < 0
            || next_guard_loc.0 > height_max
            || next_guard_loc.1 < 0
            || next_guard_loc.1 > width_max
        {
            // The guard is outside the map
            guard_outside_map = true;

            // Update the guards location as traversed
            map[guard_loc.0 as usize][guard_loc.1 as usize] = 'X';
        }
        // If the next location is blocked ('#')
        else if map[next_guard_loc.0 as usize][next_guard_loc.1 as usize] == '#' {
            // Rotate the guard
            map[guard_loc.0 as usize][guard_loc.1 as usize] =
                match map[guard_loc.0 as usize][guard_loc.1 as usize] {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => panic!("can't determine next guard shape"),
                };
        }
        // If the next location is not blocked and in bounds
        else {
            // Move the guard, mark the previous space with 'X',
            // and update the guard's location.
            map[next_guard_loc.0 as usize][next_guard_loc.1 as usize] =
                map[guard_loc.0 as usize][guard_loc.1 as usize];
            map[guard_loc.0 as usize][guard_loc.1 as usize] = 'X';
            guard_loc = (
                next_guard_loc.0.try_into().unwrap(),
                next_guard_loc.1.try_into().unwrap(),
            );
        }
    }

    find_traversed_spaces(&map)
}

/*
    ?
*/
pub fn part_two(input: Vec<String>) -> i32 {
    0
}

/*
    Given the day 6 map,
    find the starting location
    of the guard.
*/
pub fn find_guard_starting_location(map: &Vec<Vec<char>>) -> (usize, usize) {
    // Find the guard
    for height in 0..map.len() {
        for width in 0..map[height].len() {
            if map[height][width] == '^' {
                return (height, width);
            }
        }
    }

    // Not found
    (0, 0)
}

/*
    Given the day 6 map,
    find the number of unique locations
    the guard has traveled.
*/
pub fn find_traversed_spaces(map: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;

    // Find the traversed spaces
    for height in 0..map.len() {
        for width in 0..map[height].len() {
            if map[height][width] == 'X' {
                sum += 1;
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_success() {
        let input = vec![
            "....#.....".to_string(),
            ".........#".to_string(),
            "..........".to_string(),
            "..#.......".to_string(),
            ".......#..".to_string(),
            "..........".to_string(),
            ".#..^.....".to_string(),
            "........#.".to_string(),
            "#.........".to_string(),
            "......#...".to_string(),
        ];

        assert_eq!(41, part_one(input));
    }
}
