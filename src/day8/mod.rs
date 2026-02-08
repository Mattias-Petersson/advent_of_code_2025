mod point;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;
use std::{error::Error, io::BufRead};

use advent_of_code_2025::read_input;
use point::{Point, PointCoord, PointMath};

struct Circuit<'a, T> {
    connected_points: HashSet<&'a Point<T>>,
}

pub fn exercise() {
    let points: Vec<Point<i64>> = parse_input();

    part_one(&points);
}

fn parse_input<T>() -> Vec<Point<T>>
where
    T: FromStr + PointMath,
    <T as FromStr>::Err: Error + 'static,
{
    let mut input = read_input("day8").unwrap();
    points_from_input(&mut input).unwrap()
}

fn part_one<T>(points: &[Point<T>])
where
    T: PointCoord,
{
    let points_distance = point::points_squared_distance(&points);
    let circuits = make_circuits(&points_distance, 1000);
    let res: usize = circuits
        .iter()
        .take(3)
        .map(|c| c.connected_points.len())
        .product();
    println!("{:?}", res);
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

fn find_circuit<'a, T>(circuits: &[Circuit<T>], point: &Point<T>) -> Option<usize>
where
    T: PointCoord,
{
    circuits
        .iter()
        .position(|c| c.connected_points.contains(point))
}

fn make_circuits<'a, T>(
    vec_points_distance: &'a [(&'a Point<T>, &'a Point<T>, T)],
    circuit_count: usize,
) -> Vec<Circuit<'a, T>>
where
    T: PointCoord,
{
    let mut circuits: Vec<Circuit<'a, T>> = Vec::new();
    let v_iter = vec_points_distance.into_iter().take(circuit_count);
    for (p1, p2, _) in v_iter {
        let idx1 = find_circuit(&circuits, p1);
        let idx2 = find_circuit(&circuits, p2);

        match (idx1, idx2) {
            (None, None) => {
                let mut points = HashSet::new();
                points.insert(*p1);
                points.insert(*p2);
                circuits.push(Circuit {
                    connected_points: points,
                });
            }
            (Some(i), None) => {
                circuits[i].connected_points.insert(*p2);
            }
            (None, Some(j)) => {
                circuits[j].connected_points.insert(*p1);
            }
            (Some(i), Some(j)) => {
                if i != j {
                    let (hi, lo) = if i > j { (i, j) } else { (j, i) };
                    let other: Vec<&'a Point<T>> = circuits[hi].connected_points.drain().collect();
                    circuits[lo].connected_points.extend(other);
                    circuits.remove(hi);
                }
            }
        }
    }
    circuits.sort_by(|a, b| b.connected_points.len().cmp(&a.connected_points.len()));
    circuits
}
#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn setup() -> Vec<Point<i32>> {
        let file = File::open(format!("src/day8/example_input.txt")).unwrap();
        let mut reader = BufReader::new(file);

        points_from_input(&mut reader).unwrap()
    }
    #[test]
    fn test_circuits() {
        let points = setup();
        let points_distance = point::points_squared_distance(&points);
        let circuits = make_circuits(&points_distance, 10);
        let res: usize = circuits
            .iter()
            .take(3)
            .map(|c| c.connected_points.len())
            .product();
        assert_eq!(res, 40);
    }
}
