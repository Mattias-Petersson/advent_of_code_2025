use std::{fs::File, io::BufReader};

struct Problem {
    numbers: Vec<u64>,
    operator: char,
}

pub fn exercise() {
    unimplemented!()
}

fn operator_on_numbers(problem: &Problem) -> u64 {
    match problem.operator {
        '+' => unimplemented!(),
        '-' => unimplemented!(),
        '*' => unimplemented!(),
        '/' => unimplemented!(),
        _ => 0,
    }
}

fn input_to_data(input: BufReader<File>) -> Vec<Problem> {
    unimplemented!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_against_example() {
        let file = File::open(format!("src/day6/example_input.txt")).unwrap();
        let reader = BufReader::new(file);
        let problems = input_to_data(reader);
        let res: u64 = problems.into_iter().map(|p| operator_on_numbers(&p)).sum();
        assert_eq!(res, 0);
    }
}
