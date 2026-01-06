use std::{error::Error, io::BufRead};

use advent_of_code_2025::read_input;

pub fn exercise() {
    match get_batteries() {
        Ok(batteries) => {
            let sum: u64 = batteries.iter().map(|b| get_largest_joltage(b)).sum();
            println!("{}", sum);
        }
        Err(e) => eprintln!("Error {e}"),
    }
}

fn get_batteries() -> Result<Vec<String>, Box<dyn Error>> {
    let input = read_input("day3")?;
    let res = input.lines().collect::<Result<Vec<String>, _>>()?;
    Ok(res)
}

fn get_largest_joltage(batteries: &str) -> u64 {
    let stack_size = 12;
    let digits: Vec<char> = batteries.chars().collect();
    let n = digits.len();
    let mut stack: Vec<char> = Vec::new();

    for (i, &c) in digits.iter().enumerate() {
        while let Some(&top) = stack.last() {
            let remaining = n - i;
            let needed = stack_size - stack.len();

            if top < c && remaining > needed {
                stack.pop();
            } else {
                break;
            }
        }

        if stack.len() < stack_size {
            stack.push(c);
        }
    }
    stack.into_iter().collect::<String>().parse().unwrap()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_bank() {
        let joltage = get_largest_joltage("987654321111111");
        assert_eq!(joltage, 987654321111);
    }

    #[test]
    fn test_ends_bank() {
        let joltage = get_largest_joltage("811111111111119");
        assert_eq!(joltage, 811111111119);
    }

    #[test]
    fn test_arbitrary_bank() {
        let joltage = get_largest_joltage("818181911112111");
        assert_eq!(joltage, 888911112111);
    }
}
