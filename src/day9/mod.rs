use std::{error::Error, io::BufRead};

use advent_of_code_2025::read_input;

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
}

pub fn exercise() {
    match read_red_tiles() {
        Ok(res) => {
            println!("Largest area is: {}", find_largest_area(&res));
        }
        Err(e) => eprintln!("Error: {e}"),
    }
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
        let res = find_largest_area(&points);
        assert_eq!(res, 50);
    }
}
