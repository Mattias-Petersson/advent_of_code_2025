use std::{error::Error, io::BufRead};

use advent_of_code_2025::read_input;

#[derive(Debug)]
struct RedTile {
    x: u64,
    y: u64,
}

impl RedTile {
    fn new(x: u64, y: u64) -> Self {
        RedTile { x, y }
    }
    fn area_bounds(&self, p: &RedTile) -> u64 {
        let dx = self.x.abs_diff(p.x) + 1;
        let dy = self.y.abs_diff(p.y) + 1;
        dx * dy
    }

    fn is_straight_line(&self, p: &RedTile) -> bool {
        self.x == p.x || self.y == p.y
    }
}

pub fn exercise() {
    let mut res = vec![
        RedTile::new(7, 1),
        RedTile::new(11, 1),
        RedTile::new(11, 7),
        RedTile::new(9, 7),
        RedTile::new(9, 5),
        RedTile::new(2, 5),
        RedTile::new(2, 3),
        RedTile::new(7, 3),
    ];
    find_largest_green_area(&mut res);
    // match read_red_tiles() {
    //     Ok(res) => {
    //         println!("Largest area is: {}", find_largest_area(&res));
    //         let _r = find_largest_green_area(&res);
    //     }
    //     Err(e) => eprintln!("Error: {e}"),
    // }
}

fn read_red_tiles() -> Result<Vec<RedTile>, Box<dyn Error>> {
    let input = read_input("day9")?;
    input
        .lines()
        .map(|line| {
            let values: Vec<u64> = line?
                .split(',')
                .map(|s| s.parse::<u64>())
                .collect::<Result<_, _>>()?;
            Ok(RedTile::new(values[0], values[1]))
        })
        .collect()
}

fn find_largest_area(points: &[RedTile]) -> u64 {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| points.iter().skip(i + 1).map(move |p2| p1.area_bounds(p2)))
        .max()
        .unwrap_or(0)
}

fn find_fill_lines(points: &mut Vec<RedTile>) {
    let lines = find_line_segments(points);
    points.append(&mut fill_line_segments(&lines));
}

fn find_line_segments(points: &[RedTile]) -> Vec<(&RedTile, &RedTile)> {
    points
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| {
            points
                .iter()
                .skip(i + 1)
                .filter(move |p2| p1.is_straight_line(p2))
                .map(move |p2| (p1, p2))
        })
        .collect()
}

fn fill_line_segments(lines: &Vec<(&RedTile, &RedTile)>) -> Vec<RedTile> {
    let mut res = Vec::new();
    for (p1, p2) in lines {
        if p1.x == p2.x {
            let start = p1.y.min(p2.y);
            let end = p1.y.max(p2.y);
            for y in start..=end {
                res.push(RedTile { x: p1.x, y });
            }
        } else {
            let start = p1.x.min(p2.x);
            let end = p1.x.max(p2.x);
            for x in start..=end {
                res.push(RedTile { x, y: p1.y });
            }
        }
    }
    res
}

fn find_largest_green_area(points: &mut Vec<RedTile>) -> u64 {
    println!("{}", points.len());
    find_fill_lines(points);
    println!("{}", points.len());
    for p in points {
        println!("{:?}", p);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    fn setup() -> Vec<RedTile> {
        vec![
            RedTile::new(7, 1),
            RedTile::new(11, 1),
            RedTile::new(11, 7),
            RedTile::new(9, 7),
            RedTile::new(9, 5),
            RedTile::new(2, 5),
            RedTile::new(2, 3),
            RedTile::new(7, 3),
        ]
    }

    #[test]
    fn test_area() {
        let points = setup();
        let p1 = &points[1];
        let p2 = &points[5];
        let res = p1.area_bounds(p2);
        assert_eq!(res, 50);
    }
    #[test]
    fn test_largest_area() {
        let points = setup();
        assert_eq!(find_largest_area(&points), 50);
    }

    #[test]
    fn test_largest_green_area() {
        let mut points = setup();
        let p1 = &points[1];
        let p2 = &points[2];
        println!(
            "p1: {}, {} \np2: {}, {} \n Line segment: {}",
            p1.x,
            p1.y,
            p2.x,
            p2.y,
            p1.is_straight_line(p2)
        );
        assert_eq!(find_largest_green_area(&mut points), 24);
    }
}
