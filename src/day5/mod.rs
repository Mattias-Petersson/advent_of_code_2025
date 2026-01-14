use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use advent_of_code_2025::read_input;

struct FreshRange {
    low: u64,
    high: u64,
}

pub fn exercise() {
    let input = read_input("day5").unwrap();
    let (ids, fresh_ranges) = input_to_data(input);
    let fresh_count = count_fresh(ids, fresh_ranges);
    println!("Fresh count: {}", fresh_count);
}

fn input_to_data(input: BufReader<File>) -> (Vec<u64>, Vec<FreshRange>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut lines = input.lines();

    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        let parts: Vec<&str> = line.split('-').collect();
        let low = parts[0].parse::<u64>().unwrap();
        let high = parts[1].parse::<u64>().unwrap();
        ranges.push(FreshRange { low, high });
    }

    while let Some(Ok(line)) = lines.next() {
        let n = line.parse::<u64>().unwrap();
        ids.push(n);
    }
    (ids, ranges)
}

fn is_fresh(fresh_range: &FreshRange, id: u64) -> bool {
    return fresh_range.low <= id && id <= fresh_range.high;
}

fn count_fresh(ids: Vec<u64>, fresh_ranges: Vec<FreshRange>) -> u32 {
    let mut count = 0;
    'outer: for id in ids {
        for fresh_range in &fresh_ranges {
            if is_fresh(&fresh_range, id) {
                count += 1;
                continue 'outer;
            }
        }
    }
    count
}
#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;
    #[test]
    fn test_against_example() {
        let file = File::open(format!("src/day5/example_input.txt")).unwrap();
        let reader = BufReader::new(file);
        let (ids, fresh_ranges) = input_to_data(reader);
        let fresh_count = count_fresh(ids, fresh_ranges);
        assert_eq!(fresh_count, 3);
    }
}
