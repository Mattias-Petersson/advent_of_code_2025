pub fn exercise() {
    unimplemented!();
}

fn get_largest_joltage(batteries: &str) -> i32 {
    0
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
