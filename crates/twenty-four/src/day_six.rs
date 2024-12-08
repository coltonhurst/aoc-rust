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

    `map` is a 2d vector representing the map,
    where each location is a tuple (char, Vec<char>):
      -      char: the actual item in the map at that location
      - Vec<char>: if the guard has been there before, a list of
                   directions when they were there previously
*/
pub fn part_two(input: Vec<String>) -> i32 {
    let mut map: Vec<Vec<(char, Vec<char>)>> = input
        .iter()
        .map(|line| line.chars().map(|c| (c, Vec::new())).collect())
        .collect();
    let mut valid_insert_locations: Vec<(usize, usize)> = Vec::new();
    let mut guard_loc =
        find_guard_starting_location(&input.iter().map(|line| line.chars().collect()).collect());

    // Add in front of the guard as a valid location so it won't be checked,
    // we -1 for this later. (Special rule from instructions.)
    valid_insert_locations.push((guard_loc.0 - 1, guard_loc.1));

    let height_max: i32 = map.len() as i32 - 1;
    let width_max: i32 = map[0].len() as i32 - 1;
    let mut guard_outside_map = false;

    // Move the guard until the guard exits the map
    while !guard_outside_map {
        let next_guard_loc = match map[guard_loc.0][guard_loc.1].0 {
            '^' => (guard_loc.0 as i32 - 1, guard_loc.1 as i32),
            '>' => (guard_loc.0 as i32, guard_loc.1 as i32 + 1),
            'v' => (guard_loc.0 as i32 + 1, guard_loc.1 as i32),
            '<' => (guard_loc.0 as i32, guard_loc.1 as i32 - 1),
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
        }
        // If the next location is blocked ('#')
        else if map[next_guard_loc.0 as usize][next_guard_loc.1 as usize].0 == '#' {
            // Rotate the guard
            map[guard_loc.0 as usize][guard_loc.1 as usize].0 =
                match map[guard_loc.0 as usize][guard_loc.1 as usize].0 {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => panic!("can't determine next guard shape"),
                };
        }
        // If the next location is not blocked and in bounds
        else {
            // See if adding a new block in front of the guard causes a loop.
            // If so, increment valid_insert_locations
            if !valid_insert_locations
                .contains(&(next_guard_loc.0 as usize, next_guard_loc.1 as usize))
            {
                let mut changed_map = map.clone();
                changed_map[next_guard_loc.0 as usize][next_guard_loc.1 as usize].0 = '#';

                if guard_loops(changed_map) {
                    valid_insert_locations
                        .push((next_guard_loc.0 as usize, next_guard_loc.1 as usize));
                }
            }

            let guard_dir = map[guard_loc.0 as usize][guard_loc.1 as usize].0;

            // Move the guard
            map[next_guard_loc.0 as usize][next_guard_loc.1 as usize].0 = guard_dir;

            // Append the old loc & pos of the guard to the location's "memory"
            map[guard_loc.0 as usize][guard_loc.1 as usize]
                .1
                .push(guard_dir);

            // Update the guard's location
            guard_loc = (
                next_guard_loc.0.try_into().unwrap(),
                next_guard_loc.1.try_into().unwrap(),
            );
        }
    }

    valid_insert_locations.len() as i32 - 1
}

/*
    Given a map and starting location,
    determine if the guard will
    return to a location she has
    been to before, in the same direction.

    If so, it's a loop.
*/
fn guard_loops(map: Vec<Vec<(char, Vec<char>)>>) -> bool {
    let mut map = map.clone();

    let height_max: i32 = map.len() as i32 - 1;
    let width_max: i32 = map[0].len() as i32 - 1;
    let mut guard_outside_map = false;
    let mut guard_loc: (usize, usize) = (0, 0);

    // Find the guard
    for height in 0..map.len() {
        for width in 0..map[height].len() {
            if map[height][width].0 == '^'
                || map[height][width].0 == '>'
                || map[height][width].0 == 'v'
                || map[height][width].0 == '<'
            {
                guard_loc = (height, width);
            }
        }
    }

    // Move the guard until the guard exits the map
    while !guard_outside_map {
        let next_guard_loc = match map[guard_loc.0][guard_loc.1].0 {
            '^' => (guard_loc.0 as i32 - 1, guard_loc.1 as i32),
            '>' => (guard_loc.0 as i32, guard_loc.1 as i32 + 1),
            'v' => (guard_loc.0 as i32 + 1, guard_loc.1 as i32),
            '<' => (guard_loc.0 as i32, guard_loc.1 as i32 - 1),
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
        }
        // If the next location is blocked ('#')
        else if map[next_guard_loc.0 as usize][next_guard_loc.1 as usize].0 == '#' {
            // Rotate the guard
            map[guard_loc.0 as usize][guard_loc.1 as usize].0 =
                match map[guard_loc.0 as usize][guard_loc.1 as usize].0 {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => panic!("can't determine next guard shape"),
                };
        }
        // If the next location is not blocked and in bounds
        else {
            let guard_dir = map[guard_loc.0 as usize][guard_loc.1 as usize].0;

            // Have we been at the next location before, in the same direction?
            // If so, it's a loop
            if map[next_guard_loc.0 as usize][next_guard_loc.1 as usize]
                .1
                .contains(&guard_dir)
            {
                return true;
            }

            // If not...

            // Move the guard
            map[next_guard_loc.0 as usize][next_guard_loc.1 as usize].0 = guard_dir;

            // Append the old loc & pos of the guard to the location's "memory"
            map[guard_loc.0 as usize][guard_loc.1 as usize]
                .1
                .push(guard_dir);

            // Update the guard's location
            guard_loc = (
                next_guard_loc.0.try_into().unwrap(),
                next_guard_loc.1.try_into().unwrap(),
            );
        }
    }

    false
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
