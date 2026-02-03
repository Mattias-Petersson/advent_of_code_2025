use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};

#[derive(Eq, Hash, PartialEq, Debug)]
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

pub fn points_squared_distance<T>(points: &[Point<T>]) -> Vec<(&Point<T>, &Point<T>, T)>
where
    T: Copy + Ord + Mul<Output = T> + Sub<Output = T> + Add<Output = T>,
{
    let mut res = Vec::new();
    for (i, p1) in points.iter().enumerate() {
        for p2 in &points[i + 1..] {
            let dist = p1.squared_distance(p2);
            res.push((p1, p2, dist));
        }
    }
    res.sort_by_key(|e| e.2);
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_squared_distance() {
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

    #[test]
    fn test_find_closest_point() {
        let points = vec![
            Point::new(162, 817, 812),
            Point::new(425, 690, 689),
            Point::new(984, 92, 344),
        ];
        let points_distance = points_squared_distance(&points);
        let &(p1, p2, shortest_dis) = points_distance.first().unwrap();
        assert_eq!(p1, &points[0]);
        assert_eq!(p2, &points[1]);
        assert_eq!(shortest_dis, 100427);
    }

    #[test]
    fn test_find_closest_not_first() {
        let points = vec![
            Point::new(819, 987, 18),
            Point::new(57, 618, 57),
            Point::new(466, 668, 158),
            Point::new(984, 92, 344),
            Point::new(425, 690, 689),
        ];
        let points_distance = points_squared_distance(&points);
        let &(p1, p2, shortest_dis) = points_distance.first().unwrap();
        assert_eq!(p1, &points[1]);
        assert_eq!(p2, &points[2]);
        assert_eq!(shortest_dis, 179982);
    }
}
