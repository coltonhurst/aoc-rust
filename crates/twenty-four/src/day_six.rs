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

/*
    Given a square map with movement
    rules for the guard, find how many
    single blocked locations will cause the
    guard to loop infinitely.

    Map: (height, width)

     0,0 ----> (0, width_max)
     |
     |
     |
     V
    (height_max, 0)
*/
pub fn part_two(input: Vec<String>) -> i32 {
    // Create the map
    let mut map: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();

    // Valid locations where adding a # makes a loop
    let mut valid_insert_locations: Vec<(usize, usize)> = Vec::new();

    // Guard variables
    let mut current_guard_location = (0, 0);
    let mut guard_outside_map = false;

    // Find the guard
    for height in 0..map.len() {
        for width in 0..map[height].len() {
            if map[height][width] == '^' {
                current_guard_location = (height as i32, width as i32);
            }
        }
    }

    // Add in front of the guard as a valid location so it won't be checked,
    // we -1 for this later. (Special rule from instructions.)
    valid_insert_locations.push((
        current_guard_location.0 as usize - 1,
        current_guard_location.1 as usize,
    ));

    // Move the guard until the guard exits the map
    while !guard_outside_map {
        let next_guard_location =
            match map[current_guard_location.0 as usize][current_guard_location.1 as usize] {
                '^' => (
                    current_guard_location.0 as i32 - 1,
                    current_guard_location.1 as i32,
                ),
                '>' => (
                    current_guard_location.0 as i32,
                    current_guard_location.1 as i32 + 1,
                ),
                'v' => (
                    current_guard_location.0 as i32 + 1,
                    current_guard_location.1 as i32,
                ),
                '<' => (
                    current_guard_location.0 as i32,
                    current_guard_location.1 as i32 - 1,
                ),
                _ => panic!("can't determine next guard location"),
            };

        // If the next location is outside the map bounds
        if is_guard_going_out_of_bounds(&map, next_guard_location) {
            // The guard is going outside the map
            map[current_guard_location.0 as usize][current_guard_location.1 as usize] = '.';
            guard_outside_map = true;
        }
        // If the next location is blocked
        else if map[next_guard_location.0 as usize][next_guard_location.1 as usize] == '#' {
            // Rotate the guard
            map[current_guard_location.0 as usize][current_guard_location.1 as usize] =
                match map[current_guard_location.0 as usize][current_guard_location.1 as usize] {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => panic!("can't determine next guard shape"),
                };
        }
        // If the next location is not blocked and in bounds
        else {
            // Does adding a new block in front of the guard cause a loop?
            // If so, add the location to `valid_insert_locations`
            if !valid_insert_locations.contains(&(
                next_guard_location.0 as usize,
                next_guard_location.1 as usize,
            )) {
                let mut map_copy = map.clone();
                map_copy[next_guard_location.0 as usize][next_guard_location.1 as usize] = '#';

                if does_guard_loop(&map_copy, current_guard_location) {
                    valid_insert_locations.push((
                        next_guard_location.0 as usize,
                        next_guard_location.1 as usize,
                    ));
                }
            }

            // Move the guard
            map[next_guard_location.0 as usize][next_guard_location.1 as usize] =
                map[current_guard_location.0 as usize][current_guard_location.1 as usize];

            // Remove the guard from the old position
            map[current_guard_location.0 as usize][current_guard_location.1 as usize] = '.';

            // Update the guard's location
            current_guard_location = (next_guard_location.0, next_guard_location.1);
        }
    }

    valid_insert_locations.len() as i32 - 1
}

fn is_guard_going_out_of_bounds(map: &Vec<Vec<char>>, next_guard_location: (i32, i32)) -> bool {
    next_guard_location.0 < 0
        || next_guard_location.0 > (map.len() as i32 - 1)
        || next_guard_location.1 < 0
        || next_guard_location.1 > (map[0].len() as i32 - 1)
}

fn does_guard_loop(map: &Vec<Vec<char>>, current_guard_location: (i32, i32)) -> bool {
    let mut memory_map: Vec<Vec<(char, Vec<char>)>> = Vec::new();
    let mut current_guard_location = current_guard_location;
    let mut guard_outside_map = false;

    // Populate our new map that remembers if we've been to a location before
    for height in 0..map.len() {
        let mut row: Vec<(char, Vec<char>)> = Vec::new();

        for width in 0..map[height].len() {
            row.push((map[height][width], Vec::new()));
        }

        memory_map.push(row);
    }

    // Move the guard until the guard exits the map
    while !guard_outside_map {
        let next_guard_location = match memory_map[current_guard_location.0 as usize]
            [current_guard_location.1 as usize]
            .0
        {
            '^' => (
                current_guard_location.0 as i32 - 1,
                current_guard_location.1 as i32,
            ),
            '>' => (
                current_guard_location.0 as i32,
                current_guard_location.1 as i32 + 1,
            ),
            'v' => (
                current_guard_location.0 as i32 + 1,
                current_guard_location.1 as i32,
            ),
            '<' => (
                current_guard_location.0 as i32,
                current_guard_location.1 as i32 - 1,
            ),
            _ => panic!("can't determine next guard location"),
        };

        // If the next location is outside the map bounds
        if next_guard_location.0 < 0
            || next_guard_location.0 > (memory_map.len() as i32 - 1)
            || next_guard_location.1 < 0
            || next_guard_location.1 > (memory_map[0].len() as i32 - 1)
        {
            // The guard is going outside the map
            memory_map[current_guard_location.0 as usize][current_guard_location.1 as usize].0 =
                '.';
            guard_outside_map = true;
        }
        // If the next location is blocked
        else if memory_map[next_guard_location.0 as usize][next_guard_location.1 as usize].0
            == '#'
        {
            // Rotate the guard
            memory_map[current_guard_location.0 as usize][current_guard_location.1 as usize].0 =
                match memory_map[current_guard_location.0 as usize]
                    [current_guard_location.1 as usize]
                    .0
                {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => panic!("can't determine next guard shape"),
                };
        }
        // If the next location is not blocked and in bounds
        else {
            // If we have been to this or the next location before in the same direction, we are in a loop
            let current_guard_char =
                memory_map[current_guard_location.0 as usize][current_guard_location.1 as usize].0;
            if memory_map[current_guard_location.0 as usize][current_guard_location.1 as usize]
                .1
                .contains(&current_guard_char)
                || memory_map[next_guard_location.0 as usize][next_guard_location.1 as usize]
                    .1
                    .contains(&current_guard_char)
            {
                return true;
            }

            // Move the guard
            memory_map[next_guard_location.0 as usize][next_guard_location.1 as usize].0 =
                memory_map[current_guard_location.0 as usize][current_guard_location.1 as usize].0;

            // Remove the guard from the old position
            memory_map[current_guard_location.0 as usize][current_guard_location.1 as usize].0 =
                '.';

            // Remember that we've been here before
            memory_map[current_guard_location.0 as usize][current_guard_location.1 as usize]
                .1
                .push(current_guard_char);

            // Update the guard's location
            current_guard_location = (next_guard_location.0, next_guard_location.1);
        }
    }

    false
}

pub fn print_map(map: &Vec<Vec<(char, Vec<char>)>>) {
    for height in 0..map.len() {
        for width in 0..map[height].len() {
            print!("{:?}", map[height][width].0);
        }
        println!("");
    }
    println!("");
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

    #[test]
    fn part_two_success() {
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

        assert_eq!(6, part_two(input));
    }
}
