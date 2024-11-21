use std::path::Path;
use utils::read_input_file;

fn main() {
    let input_file_path = Path::new("./input-files/23-1.txt");
    let input = read_input_file(input_file_path);

    println!(
        "Day One, Part One: {:?}",
        twenty_three::day_one::part_one(input.clone())
    );
    println!(
        "Day One, Part Two: {:?}",
        twenty_three::day_one::part_two(input)
    );
}
