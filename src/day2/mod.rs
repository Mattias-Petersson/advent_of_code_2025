type IntervalIds = (u32, u32);

pub fn exercise() {
    unimplemented!();
}

fn temp(input: &str) -> i64 {
    0
}

fn find_sum_invalid_ids(interval: IntervalIds) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_against_example() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = temp(input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn validate_sum_invalid() {
        let interval: IntervalIds = (11, 22);
        let result = find_sum_invalid_ids(interval);
        assert_eq!(result, 33);
    }
}
