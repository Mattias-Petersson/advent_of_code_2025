use std::ops::{Add, Mul, Sub};

pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
impl<T> Point<T>
where
    T: Copy + Mul<Output = T> + Sub<Output = T> + Add<Output = T>,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Point { x, y, z }
    }
    pub fn squared_distance(&self, p2: &Point<T>) -> T {
        let dx = p2.x - self.x;
        let dy = p2.y - self.y;
        let dz = p2.z - self.z;

        dx * dx + dy * dy + dz * dz
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_squared_difference() {
        let p1 = Point::new(162, 817, 812);
        let p2 = Point::new(425, 690, 689);

        let sq_diff: i32 = p1.squared_distance(&p2);
        assert_eq!(sq_diff, 100427);
    }

    #[test]
    fn test_distance_to_self() {
        let p1 = Point::new(2, 4, 9);
        let dist = p1.squared_distance(&p1);
        assert_eq!(dist, 0);
    }
}
