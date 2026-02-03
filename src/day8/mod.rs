mod point;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use std::{error::Error, io::BufRead};

use advent_of_code_2025::read_input;
use point::Point;

pub fn exercise() {
    let mut input = read_input("day8").unwrap();
    let points: Vec<Point<i64>> = points_from_input(&mut input).unwrap();
    let distances = point::points_squared_distance(&points).unwrap();
    println!("{:?}", distances);
}

fn points_from_input<T>(input: &mut BufReader<File>) -> Result<Vec<Point<T>>, Box<dyn Error>>
where
    T: FromStr + Copy + Sub<Output = T> + Add<Output = T> + Mul<Output = T>,
    <T as FromStr>::Err: Error + 'static,
{
    let mut points = Vec::new();
    for line in input.lines() {
        let line = line?;
        let nums: Vec<T> = line
            .split(',')
            .map(|s| s.parse::<T>())
            .collect::<Result<_, _>>()?;
        points.push(Point::new(nums[0], nums[1], nums[2]));
    }
    Ok(points)
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn setup() -> Result<Vec<Point<i32>>, Box<dyn Error>> {
        let file = File::open(format!("src/day8/example_input.txt"))?;
        let mut reader = BufReader::new(file);

        points_from_input(&mut reader)
    }
    #[test]
    fn test_circuits() {
        let points = setup().unwrap();

        let res = 0;
        assert_eq!(res, 40);
        unimplemented!();
    }
}
