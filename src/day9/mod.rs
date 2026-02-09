struct RedTile {
    x: u32,
    y: u32,
}

impl RedTile {
    fn new(x: u32, y: u32) -> Self {
        RedTile { x, y }
    }
    fn area(&self, p: &RedTile) -> u32 {
        let dx = self.x.abs_diff(p.x) + 1;
        let dy = self.y.abs_diff(p.y) + 1;
        dx * dy
    }
}

pub fn exercise() {
    unimplemented!();
}

fn find_largest_area(points: &[RedTile]) -> u32 {
    unimplemented!();
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
        let res = p1.area(p2);
        assert_eq!(res, 50);
    }
    #[test]
    fn test_largest_area() {
        let points = setup();
        let res = find_largest_area(&points);
        assert_eq!(res, 50);
    }
}
