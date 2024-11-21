use std::{
    fs::File,
    io::{BufRead, BufReader},
    panic,
    path::Path,
};

pub fn read_input_file(filename: &Path) -> Vec<String> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => panic!("Error opening input file."),
    };

    let file_buffer = BufReader::new(file);

    file_buffer
        .lines()
        .map(|line| match line {
            Ok(l) => l,
            Err(_) => panic!("Couldn't read line in input file."),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_file_success() {
        let filename = Path::new("../../test-files/test.txt");
        let result = read_input_file(filename);

        assert_eq!(result, vec!["Line 1", "Line 2", "Line 3"])
    }
}
