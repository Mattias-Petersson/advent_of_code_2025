use std::fs;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
pub struct Point {
    x: u64,
    y: u64,
}

impl Point {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }

    pub fn area(&self, p: &Point) -> u64 {
        let delta_x = p.x.abs_diff(self.x) + 1;
        let delta_y = p.y.abs_diff(self.y) + 1;
        delta_x * delta_y
    }
}
pub fn read_input() -> Result<Vec<Point>, Box<dyn std::error::Error>> {
    let input_str = fs::read_to_string("src/day9/example_input.txt")?;
    input_str
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").ok_or("No delimiter")?;
            let p = Point::new(x.parse()?, y.parse()?);
            Ok(p)
        })
        .collect::<Result<Vec<Point>, _>>()
}

pub fn add_inbetween_points(orig_list: &[Point]) -> Vec<Point> {
    orig_list
        .windows(2)
        .map(|w| (&w[0], &w[1]))
        .chain(std::iter::once((
            &orig_list[orig_list.len() - 1],
            &orig_list[0],
        )))
        .flat_map(|(p1, p2)| interpolate(p1, p2))
        .collect()
}

fn interpolate(p1: &Point, p2: &Point) -> Vec<Point> {
    if p1.x != p2.x {
        let (begin, end) = (p1.x.min(p2.x), p1.x.max(p2.x));
        (begin..end).map(|x| Point::new(x, p1.y)).collect()
    } else {
        let (begin, end) = (p1.y.min(p2.y), p1.y.max(p2.y));
        (begin..end).map(|y| Point::new(p1.x, y)).collect()
    }
}

pub fn largest_area_between_all_points(all_p: &[Point]) -> u64 {
    all_p
        .par_iter()
        .enumerate()
        .map(|(i, p1)| {
            all_p[i + 1..]
                .iter()
                .map(|p2| p1.area(p2))
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() -> Vec<Point> {
        vec![
            Point::new(7, 1),
            Point::new(11, 1),
            Point::new(11, 7),
            Point::new(9, 7),
            Point::new(9, 5),
            Point::new(2, 5),
            Point::new(2, 3),
            Point::new(7, 3),
        ]
    }

    #[test]
    fn test_area() {
        let points = setup();
        let p1 = &points[1];
        let p2 = &points[5];
        let res = p1.area(p2);
        assert_eq!(res, 50);
    }
    #[test]
    fn test_largest_area() {
        let points = setup();
        let res = largest_area_between_all_points(&points);
        assert_eq!(res, 50);
    }
}
