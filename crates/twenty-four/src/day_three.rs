use regex::Regex;

/*
    add info
*/
pub fn part_one(input: Vec<String>) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut sum = 0;

    for line in input {
        let valid_texts: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();
        for mul in valid_texts {
            let numbers: Vec<&str> = mul[4..mul.len() - 1].split(',').collect();

            sum += numbers.first().unwrap().parse::<i32>().unwrap()
                * numbers.last().unwrap().parse::<i32>().unwrap();
        }
    }

    sum
}
