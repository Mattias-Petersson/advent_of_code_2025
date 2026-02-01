mod point;

pub use point::Point;

pub fn exercise() {
    let p1 = Point::new(0, 0, 0);
    let res = p1.squared_distance(&p1);
    println!("Distance to self: {}", res);
    unimplemented!();
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_circuits() {
        let res = 0;
        assert_eq!(res, 40);
        unimplemented!();
    }
}
