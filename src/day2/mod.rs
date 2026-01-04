use std::{
    error::Error,
    fs::File,
    io::{BufReader, Read},
};

struct Interval {
    start: i64,
    end: i64,
}

pub fn exercise() {
    match read_intervals() {
        Ok(data) => {
            let invalid_sum = sum_invalid_ids(data);
            println!("{}", invalid_sum);
        }
        Err(e) => eprintln!("Unexpected error: {}", e),
    }
}

fn read_intervals() -> Result<Vec<Interval>, Box<dyn Error>> {
    let mut reader = read_input()?;
    let mut input_str = String::new();
    reader.read_to_string(&mut input_str)?;
    parse_intervals(&input_str)
}

fn parse_intervals(input: &str) -> Result<Vec<Interval>, Box<dyn Error>> {
    input
        .split(",")
        .map(|s| s.trim())
        .map(|interval| {
            let mut parts = interval.split("-");
            let start = parts.next().unwrap().parse::<i64>()?;
            let end = parts.next().unwrap().parse::<i64>()?;
            Ok(Interval { start, end })
        })
        .collect()
}

fn sum_invalid_ids(intervals: Vec<Interval>) -> i64 {
    let mut sum = 0;
    for interval in intervals {
        for i in interval.start..=interval.end {
            let i_str = i.to_string();
            let i_len = i_str.len();

            let mid = i_len / 2;
            let first_half = &i_str[..mid];
            let second_half = &i_str[mid..];

            if first_half == second_half {
                sum += i
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_against_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let intervals = parse_intervals(input).unwrap();
        let result = sum_invalid_ids(intervals);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_sum_11_22() {
        let interval = Interval { start: 11, end: 22 };
        let intervals = vec![interval];
        let result = sum_invalid_ids(intervals);
        assert_eq!(result, 33);
    }
}

fn read_input() -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open("src/day2/input.txt")?;
    Ok(BufReader::new(file))
}
