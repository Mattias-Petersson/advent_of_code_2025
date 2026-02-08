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
    let fresh_count = count_fresh(ids, &fresh_ranges);
    let ids_fresh = fresh_range_count(fresh_ranges);
    println!("Fresh count: {}, id count {}", fresh_count, ids_fresh);
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

fn is_id_fresh(fresh_range: &FreshRange, id: u64) -> bool {
    fresh_range.low <= id && id <= fresh_range.high
}

fn count_fresh(ids: Vec<u64>, fresh_ranges: &[FreshRange]) -> u32 {
    ids.iter()
        .filter(|&id| fresh_ranges.iter().any(|range| is_id_fresh(range, *id)))
        .count() as u32
}

fn fresh_range_count(mut fresh_ranges: Vec<FreshRange>) -> u64 {
    fresh_ranges.sort_by_key(|r| r.low);

    let mut merged = Vec::new();
    let mut current = FreshRange {
        low: fresh_ranges[0].low,
        high: fresh_ranges[0].high,
    };

    for s in fresh_ranges.iter().skip(1) {
        let range = s;
        if range.low <= current.high + 1 {
            current.high = current.high.max(range.high);
        } else {
            merged.push(current);
            current = FreshRange {
                low: range.low,
                high: range.high,
            };
        }
    }
    merged.push(current);

    merged.iter().map(|r| r.high - r.low + 1).sum()
}
#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn setup() -> (Vec<u64>, Vec<FreshRange>) {
        let file = File::open("src/day5/example_input.txt").unwrap();
        let reader = BufReader::new(file);
        input_to_data(reader)
    }
    #[test]
    fn test_against_example() {
        let (ids, fresh_ranges) = setup();
        let fresh_count = count_fresh(ids, &fresh_ranges);
        assert_eq!(fresh_count, 3);
    }
    #[test]
    fn test_fresh_range_count() {
        let (_ids, fresh_ranges) = setup();
        let fresh_count = fresh_range_count(fresh_ranges);
        assert_eq!(fresh_count, 14);
    }
}
