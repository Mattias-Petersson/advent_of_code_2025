use std::{fs::File, io::BufReader};

pub fn exercise() {
    unimplemented!();
}

fn input_to_grid(input: BufReader<File>) -> Vec<Vec<char>> {
    vec![vec!['0']]
}

fn calculate_accessible(grid: Vec<Vec<char>>) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_against_example() {
        let file = File::open(format!("src/day4/example_input.txt")).unwrap();
        let reader = BufReader::new(file);
        let grid = input_to_grid(reader);
        let no_accessible = calculate_accessible(grid);
        assert_eq!(no_accessible, 13);
    }
}
