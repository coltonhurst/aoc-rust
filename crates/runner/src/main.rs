use std::path::Path;
use utils::read_input_file;

fn main() {
    twenty_four_solutions();
}

fn twenty_four_solutions() {
    // Day 1
    let day_one_file = Path::new("./input-files/24-1.txt");
    let day_one_input = read_input_file(day_one_file);
    println!(
        "24-1-1: {:?}",
        twenty_four::day_one::part_one(day_one_input.clone())
    );
    println!(
        "24-1-2: {:?}",
        twenty_four::day_one::part_two(day_one_input)
    );

    // Day 2
    let day_two_file = Path::new("./input-files/24-2.txt");
    let day_two_input = read_input_file(day_two_file);
    println!(
        "24-2-1: {:?}",
        twenty_four::day_two::part_one(day_two_input.clone())
    );
    println!(
        "24-2-2: {:?}",
        twenty_four::day_two::part_two(day_two_input)
    );
}

#[allow(dead_code)]
fn twenty_three_solutions() {
    // Day 1
    let day_one_file = Path::new("./input-files/23-1.txt");
    let day_one_input = read_input_file(day_one_file);
    println!(
        "23-1-1: {:?}",
        twenty_three::day_one::part_one(day_one_input.clone())
    );
    println!(
        "23-1-2: {:?}",
        twenty_three::day_one::part_two(day_one_input)
    );

    // Day 2
    let day_two_file = Path::new("./input-files/23-2.txt");
    let day_two_input = read_input_file(day_two_file);
    println!(
        "23-2-1: {:?}",
        twenty_three::day_two::part_one(day_two_input.clone())
    );
    // println!(
    //     "23-2-2: {:?}",
    //     twenty_three::day_two::part_two(day_two_input)
    // );
}

#[allow(dead_code)]
fn fifteen_solutions() {
    // Day 1
    let day_one_file = Path::new("./input-files/15-1.txt");
    let day_one_input = read_input_file(day_one_file);
    println!(
        "15-1-1: {:?}",
        fifteen::day_one::part_one(day_one_input.clone())
    );
    println!("15-1-2: {:?}", fifteen::day_one::part_two(day_one_input));
}
