use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::read_input;

#[derive(Debug)]
struct Problem {
    numbers: Vec<u64>,
    operator: char,
}

pub fn exercise() {
    let data = read_input("day6").unwrap();
    match input_to_problems(data) {
        Ok(problems) => {
            let sum: u64 = problems.iter().map(|p| operator_on_numbers(p)).sum();
            println!("{}", sum);
        }
        Err(e) => eprintln!("Error {e}"),
    }
}

fn operator_on_numbers(problem: &Problem) -> u64 {
    match problem.operator {
        '+' => problem.numbers.iter().sum(),
        '*' => problem.numbers.iter().product(),
        _ => 0,
    }
}

fn input_to_problems(input: BufReader<File>) -> Result<Vec<Problem>, Box<dyn Error>> {
    let mut problems = Vec::new();
    let grid: Vec<Vec<String>> = input
        .lines()
        .map(|line| line.map(|l| l.split_whitespace().map(|s| s.to_string()).collect()))
        .collect::<Result<_, _>>()?;

    let row_len = grid.len();
    let col_len = grid[0].len();

    for col in 0..col_len {
        let mut numbers: Vec<u64> = Vec::new();
        for row in 0..row_len - 1 {
            numbers.push(grid[row][col].parse()?);
        }
        let operator: char = grid[row_len - 1][col].parse()?;
        problems.push(Problem { numbers, operator });
    }
    Ok(problems)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_against_example() {
        let file = File::open(format!("src/day6/example_input.txt")).unwrap();
        let reader = BufReader::new(file);

        match input_to_problems(reader) {
            Ok(problems) => {
                let res: u64 = problems.into_iter().map(|p| operator_on_numbers(&p)).sum();
                assert_eq!(res, 4277556);
            }
            Err(e) => panic!("Test in day 6 failed. Error {e}"),
        }
    }
}
