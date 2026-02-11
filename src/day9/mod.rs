use std::{collections::HashSet, error::Error, io::BufRead};

use advent_of_code_2025::read_input;

#[derive(Debug, Clone)]
struct RedTile {
    x: u64,
    y: u64,
}

struct Coordinate {
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
    // let mut res = vec![
    //     RedTile::new(7, 1),
    //     RedTile::new(11, 1),
    //     RedTile::new(11, 7),
    //     RedTile::new(9, 7),
    //     RedTile::new(9, 5),
    //     RedTile::new(2, 5),
    //     RedTile::new(2, 3),
    //     RedTile::new(7, 3),
    // ];
    // find_largest_green_area(&mut res);
    match read_red_tiles() {
        Ok(res) => {
            println!("Largest area is: {}", find_largest_area(&res));
            let _r = find_largest_green_area(&res);
            println!("{}", _r);
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

fn build_perimeter(points: &[RedTile]) -> HashSet<(u64, u64)> {
    let mut res = HashSet::new();

    for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            if !p1.is_straight_line(p2) {
                continue;
            }

            if p1.x == p2.x {
                let start = p1.y.min(p2.y);
                let end = p1.y.max(p2.y);
                for y in start..=end {
                    res.insert((p1.x, y));
                }
            } else {
                let start = p1.x.min(p2.x);
                let end = p1.x.max(p2.x);
                for x in start..=end {
                    res.insert((x, p1.y));
                }
            }
        }
    }

    res
}

fn fill_inside_perimeter(perimeter: &HashSet<(u64, u64)>) -> HashSet<(u64, u64)> {
    let min_x = perimeter.iter().map(|(x, _)| *x).min().unwrap();
    let max_x = perimeter.iter().map(|(x, _)| *x).max().unwrap();
    let min_y = perimeter.iter().map(|(_, y)| *y).min().unwrap();
    let max_y = perimeter.iter().map(|(_, y)| *y).max().unwrap();

    let mut filled = perimeter.clone();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if is_inside_fence(
                &filled,
                Coordinate { x, y },
                Coordinate { x: min_x, y: min_y },
                Coordinate { x: max_x, y: max_y },
            ) {
                filled.insert((x, y));
            }
        }
    }

    filled
}

fn is_inside_fence(
    grid: &HashSet<(u64, u64)>,
    curr: Coordinate,
    min: Coordinate,
    max: Coordinate,
) -> bool {
    let has_left = (min.x..curr.x).any(|test_x| grid.contains(&(test_x, curr.y)));
    let has_right = ((curr.x + 1)..=max.x).any(|test_x| grid.contains(&(test_x, curr.y)));
    let has_above = (min.y..curr.y).any(|test_y| grid.contains(&(curr.x, test_y)));
    let has_below = ((curr.y + 1)..=max.y).any(|test_y| grid.contains(&(curr.x, test_y)));

    has_left && has_right && has_above && has_below
}

fn find_largest_green_area(points: &[RedTile]) -> u64 {
    let perimeter = build_perimeter(points);
    println!("Got to filled perimeter");
    let point_coords = fill_inside_perimeter(&perimeter);
    println!("Filled inside the perimeter");
    let mut max_area = u64::MIN;
    'outer: for (i, p1) in points.iter().enumerate() {
        for p2 in points.iter().skip(i + 1) {
            if p1.area_bounds(p2) <= max_area {
                continue;
            }
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);

            for x_idx in min_x..=max_x {
                for y_idx in min_y..=max_y {
                    if !point_coords.contains(&(x_idx, y_idx)) {
                        continue 'outer;
                    }
                }
            }
            let area = p1.area_bounds(p2);
            max_area = max_area.max(area);
        }
    }

    max_area
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
    fn test_perimeter_fill() {
        let points = setup();
        let perimeter = build_perimeter(&points);
        assert_eq!(fill_inside_perimeter(&perimeter).len(), 46);
    }

    #[test]
    fn test_largest_green_area() {
        let points = setup();
        assert_eq!(find_largest_green_area(&points), 24);
    }
}
