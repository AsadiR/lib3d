use std::ops::Add;
use std::ops::Sub;
use std::fmt;
use base_objects::vector::Vector;
use base_objects::eq_f64;


pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl<'a,'b> Add<&'b Vector> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Vector) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

// arguments shouldn't be moved
impl<'a,'b> Sub<&'b Point> for &'a Point {
    type Output = Vector;
    fn sub(self, other: &'b Point) -> Vector {
        Vector { x: other.x - self.x, y: other.y - self.y, z: other.z - self.z}
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        eq_f64(self.x, other.x) & eq_f64(self.y, other.y) & eq_f64(self.z, other.z)
    }
}
impl Eq for Point {}


#[cfg(test)]
mod tests {
    use super::super::vector::Vector;
    use super::super::point::Point;

    #[test]
    fn point_plus_vector() {
        let p = Point {x: 1.0, y: 1.0, z: 1.0};
        let v = Vector {x: 1.0, y: 1.0, z: 1.0};
        let new_p1 = &p + &v;
        let new_p2 = &p + &v;
        let expected_p = Point {x: 2.0, y: 2.0, z: 2.0};
        assert!(new_p1 == new_p2);
        assert!(new_p1 == expected_p);
    }

    #[test]
    fn point_subtract_point() {
        let end = Point {x: 1.0, y: 1.0, z: 1.0};
        let begin = Point {x: 2.0, y: 2.0, z: 2.0};
        let v = &end - &begin;
        let new_v = &end - &begin;
        let expected_v = Vector {x: 1.0, y: 1.0, z: 1.0};
        assert!(v == new_v);
        assert!(v == expected_v);
    }
}
