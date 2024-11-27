// /*
//     For each line (game) of input, find the max
//     potential number of cubes for each color.

//     Then, determine if a game is possible or
//     impossible. If the number of cubes for
//     any color is over the 12/13/14 consts,
//     it's impossible. We are checking to see
//     if it's possible that the bag has that
//     exact number of colored cubes.

//     Sum the game id's of possible games.
// */
// pub fn part_one(input: Vec<String>) -> u32 {
//     const RED_CUBES: u32 = 12;
//     const GREEN_CUBES: u32 = 13;
//     const BLUE_CUBES: u32 = 14;

//     // (game #, max red, max blue, max green)
//     // Where max is the highest number of cubes for that
//     // color found in that game
//     let game_stats: Vec<(u32, u32, u32, u32)> = Vec::new();

//     // Loop through each game
//     for line in input {
//         // Final game numbers
//         let mut game_number: u32 = 0;
//         let mut max_red: u32 = 0;
//         let mut max_blue: u32 = 0;
//         let mut max_green: u32 = 0;

//         // Parse the line
//         let line_parts: Vec<&str> = line.split(&[':', ';', ',', ' '][..]).collect();

//         // Game #
//         if let Ok(number) = line_parts[1].parse::<u32>() {
//             game_number = number;
//         }

//         // Get max # for each color from each attempt
//         for (index, part) in line_parts.iter().enumerate() {
//             match part {
//                 "red" => {
                    
//                 },
//                 "blue" => {

//                 },
//                 "green" => {

//                 },
//                 _ => (),
//             }
//         }

//         game_stats.push((line_parts[1], ));
//     }

//     0
// }