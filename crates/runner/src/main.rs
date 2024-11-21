use std::path::Path;

use twenty_three::day_one;
use utils::read_input_file;

fn main() {
    let input_file_path = Path::new("./input-files/23-1-1.txt");
    let input = read_input_file(&input_file_path);
    println!("Day One, Part One: {:?}", day_one::part_one(input));
}
