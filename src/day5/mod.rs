use std::{fs::File, io::BufReader};

struct FreshRange {
    low: u64,
    high: u64,
}

pub fn exercise() {}

fn input_to_data(input: BufReader<File>) -> (Vec<u64>, Vec<FreshRange>) {
    unimplemented!();
}

fn is_fresh(fresh_range: &FreshRange, id: u64) -> bool {
    return fresh_range.low <= id && id <= fresh_range.high;
}
#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;
    #[test]
    fn test_against_example() {
        let mut count = 0;
        let file = File::open(format!("src/day5/example_input.txt")).unwrap();
        let reader = BufReader::new(file);
        let (ids, fresh_ranges) = input_to_data(reader);
        'outer: for id in ids {
            for fresh_range in &fresh_ranges {
                if is_fresh(&fresh_range, id) {
                    count += 1;
                    break 'outer;
                }
            }
        }
        assert_eq!(count, 3);
    }
}
