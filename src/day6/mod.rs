use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
};

use advent_of_code_2025::read_input;

#[derive(Debug)]
struct Problem {
    numbers: Vec<String>,
    operator: char,
}

enum Orientation {
    Row,
    Column,
}

pub fn exercise() {
    let data = read_input("day6").unwrap();
    match input_to_problems(data) {
        Ok(problems) => {
            let sum_over_problems: u64 = problems
                .iter()
                .filter_map(|p| solve(p, Orientation::Row).ok())
                .sum();
            println!("Sum of all problems is {}", sum_over_problems);
            let sum_cephalopod_op: u64 = problems
                .iter()
                .filter_map(|p| solve(p, Orientation::Column).ok())
                .sum();
            println!("Cephalopod math sum gives a sum of {}", sum_cephalopod_op);
        }
        Err(e) => eprintln!("Error {e}"),
    }
}
fn solve(problem: &Problem, orientation: Orientation) -> Result<u64, Box<dyn Error>> {
    let mut res = if problem.operator == '*' { 1 } else { 0 };

    match orientation {
        Orientation::Row => {
            for x in problem
                .numbers
                .iter()
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
            {
                let x = x.parse::<u64>()?;
                res = op(problem.operator, res, x);
            }
        }

        Orientation::Column => {
            let rows: Vec<&[u8]> = problem.numbers.iter().map(|s| s.as_bytes()).collect();
            let max_len = rows.iter().map(|r| r.len()).max().unwrap_or(0);

            for col in 0..max_len {
                let mut s = String::new();
                for row in &rows {
                    if col < row.len() && row[col] != b' ' {
                        s.push(row[col] as char);
                    }
                }
                if s.is_empty() {
                    continue;
                }
                let x = s.parse::<u64>()?;

                res = op(problem.operator, res, x)
            }
        }
    }

    Ok(res)
}

fn op(operator: char, acc: u64, x: u64) -> u64 {
    match operator {
        '+' => acc + x,
        '*' => acc * x,
        _ => acc,
    }
}

fn input_to_problems(input: BufReader<File>) -> Result<Vec<Problem>, Box<dyn Error>> {
    let mut problems = Vec::new();
    let lines: Vec<String> = input.lines().collect::<Result<_, _>>()?;

    let row_len = lines.len();
    let col_len = lines[0].len();

    let operator_row = lines[row_len - 1].as_bytes();

    let col_ranges = find_ranges(operator_row, col_len);
    for range in col_ranges {
        let mut numbers = Vec::new();

        for row in 0..row_len - 1 {
            let line = lines[row].as_bytes();

            let slice = if range.end <= line.len() {
                &line[range.clone()]
            } else if range.start < line.len() {
                &line[range.start..line.len()]
            } else {
                continue;
            };

            numbers.push(String::from_utf8_lossy(slice).to_string());
        }

        let operator = operator_row[range.start] as char;

        problems.push(Problem { numbers, operator });
    }
    Ok(problems)
}

fn find_ranges(row: &[u8], grid_width: usize) -> Vec<Range<usize>> {
    let column_starts: Vec<usize> = row
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| if b != b' ' { Some(i) } else { None })
        .collect();

    let mut column_ranges = Vec::new();

    for i in 0..column_starts.len() {
        let start = column_starts[i];
        let end = column_starts.get(i + 1).copied().unwrap_or(grid_width);

        column_ranges.push(start..end);
    }
    column_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Result<Vec<Problem>, Box<dyn Error>> {
        let file = File::open(format!("src/day6/example_input.txt")).unwrap();
        let reader = BufReader::new(file);

        input_to_problems(reader)
    }

    #[test]
    fn test_against_example() {
        let problems_res = setup();
        if let Ok(problems) = problems_res {
            let res: u64 = problems
                .into_iter()
                .filter_map(|p| solve(&p, Orientation::Row).ok())
                .sum();
            assert_eq!(res, 4277556);
        } else {
            let e = problems_res.unwrap_err();
            panic!("Test in day 6 failed. Error {e}");
        }
    }

    #[test]
    fn test_cephalopod_operator() {
        let problems_res = setup();
        if let Ok(problems) = problems_res {
            let res: u64 = problems
                .iter()
                .filter_map(|p| solve(p, Orientation::Column).ok())
                .sum();
            assert_eq!(res, 3263827);
        } else {
            let e = problems_res.unwrap_err();
            panic!("Cephalopod math operator went wrong, error {e}");
        }
    }
}
