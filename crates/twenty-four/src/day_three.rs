use regex::Regex;

/*
    Given input, find valid pairs of mul(#,#)
    where # is a 1 to 3 digit number.

    "Solve" (multiply) the valid pairs and sum
    the products.
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

/*
    Given input, find valid pairs of mul(#,#)
    where # is a 1 to 3 digit number.

    "Solve" (multiply) the valid pairs, unless
    you find "don't()". If you see "do()" again
    you can resume the multiplying & summing.
*/
pub fn part_two(input: Vec<String>) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let mut include_text = true;
    let mut sum = 0;

    for line in input {
        let valid_texts: Vec<&str> = re.find_iter(&line).map(|m| m.as_str()).collect();
        for valid_text in valid_texts {
            match valid_text {
                "do()" => include_text = true,
                "don't()" => include_text = false,
                _ => {
                    if include_text {
                        let numbers: Vec<&str> =
                            valid_text[4..valid_text.len() - 1].split(',').collect();
                        sum += numbers.first().unwrap().parse::<i32>().unwrap()
                            * numbers.last().unwrap().parse::<i32>().unwrap();
                    }
                }
            }
        }
    }

    sum
}
