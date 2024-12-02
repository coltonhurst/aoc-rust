/*
    Loop through each line of input.
    Determine if a game is possible or impossible.

    If the number of cubes for any color
    is over the 12/13/14 consts,
    it's impossible. We are checking to see
    if the game is possible given the following
    number of cubes in the bag:
    - 12 red cubes
    - 13 green cubes
    - 14 blue cubes

    Sum the game id's of possible games.

    Example line: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
*/
pub fn part_one(input: Vec<String>) -> u32 {
    const RED_CUBES: u32 = 12;
    const GREEN_CUBES: u32 = 13;
    const BLUE_CUBES: u32 = 14;

    let mut valid_games: Vec<u32> = Vec::new();
    let mut sum: u32 = 0;

    for line in input {
        let line_parts: Vec<&str> = line.split(&[' ', ':', ',', ';'][..]).collect();
        let game_number = line_parts[1];
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for (index, part) in line_parts.iter().enumerate() {
            match *part {
                "red" => {
                    let num_red: u32 = line_parts[index - 1].parse().unwrap();
                    if num_red > max_red {
                        max_red = num_red;
                    }
                }
                "blue" => {
                    let num_blue: u32 = line_parts[index - 1].parse().unwrap();
                    if num_blue > max_blue {
                        max_blue = num_blue;
                    }
                }
                "green" => {
                    let num_green: u32 = line_parts[index - 1].parse().unwrap();
                    if num_green > max_green {
                        max_green = num_green;
                    }
                }
                _ => (),
            }
        }

        if max_red <= RED_CUBES && max_blue <= BLUE_CUBES && max_green <= GREEN_CUBES {
            valid_games.push(game_number.parse().unwrap());
        }
    }

    for game in valid_games {
        sum += game;
    }

    sum
}
