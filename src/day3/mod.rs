use std::{error::Error, io::BufRead};

use advent_of_code_2025::read_input;

pub fn exercise() {
    match get_batteries() {
        Ok(batteries) => {
            let sum: u32 = batteries.iter().map(|b| get_largest_joltage(b)).sum();
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

fn get_largest_joltage(batteries: &str) -> u32 {
    let mut best_pair = 0;
    let mut best_first = 0;

    for c in batteries.chars() {
        let d = c.to_digit(10).unwrap();
        best_pair = best_pair.max(best_first * 10 + d);
        best_first = best_first.max(d);
    }

    best_pair
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent_bank() {
        let joltage = get_largest_joltage("987654321111111");
        assert_eq!(joltage, 98);
    }

    #[test]
    fn test_ends_bank() {
        let joltage = get_largest_joltage("811111111111119");
        assert_eq!(joltage, 89);
    }

    #[test]
    fn test_arbitrary_bank() {
        let joltage = get_largest_joltage("818181911112111");
        assert_eq!(joltage, 92);
    }

    #[test]
    fn test_sum_banks() {
        let first_joltage = get_largest_joltage("987654321111111");
        let second_joltage = get_largest_joltage("811111111111119");
        assert_eq!(first_joltage + second_joltage, 98 + 89);
    }
}
