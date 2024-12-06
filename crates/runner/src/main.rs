use std::path::Path;
use utils::read_input_file;

fn main() {
    twenty_four_solutions();
    //twenty_three_solutions();
    //fifteen_solutions();
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

    // Day 3
    let day_three_file = Path::new("./input-files/24-3.txt");
    let day_three_input = read_input_file(day_three_file);
    println!(
        "24-3-1: {:?}",
        twenty_four::day_three::part_one(day_three_input.clone())
    );
    println!(
        "24-3-2: {:?}",
        twenty_four::day_three::part_two(day_three_input)
    );

    // Day 4
    let day_four_file = Path::new("./input-files/24-4.txt");
    let day_four_input = read_input_file(day_four_file);
    println!(
        "24-4-1: {:?}",
        twenty_four::day_four::part_one(day_four_input.clone())
    );
    println!(
        "24-4-2: {:?}",
        twenty_four::day_four::part_two(day_four_input)
    );

    // Day 5
    let day_five_file = Path::new("./input-files/24-5.txt");
    let day_five_input = read_input_file(day_five_file);
    println!(
        "24-5-1: {:?}",
        twenty_four::day_five::part_one(day_five_input.clone())
    );
    println!(
        "24-5-2: {:?}",
        twenty_four::day_five::part_two(day_five_input)
    );

    // Day 6
    let day_six_file = Path::new("./input-files/24-6.txt");
    let day_six_input = read_input_file(day_six_file);
    println!(
        "24-6-1: {:?}",
        twenty_four::day_six::part_one(day_six_input.clone())
    );
    println!(
        "24-6-2: {:?}",
        twenty_four::day_six::part_two(day_six_input)
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
