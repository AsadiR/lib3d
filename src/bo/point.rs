use std::ops::Add;
use std::ops::Sub;
use std::fmt;
use bo::vector::Vector;
use bo::eq_f32;
use bo::base_object::BaseObject;
use std::cmp::Ordering;

#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Point {
    pub fn convert_to_vector(self) -> Vector {
        Vector {x: self.x, y: self.y, z: self.z}
    }
    pub fn new(x : f32, y : f32, z : f32) -> Point {
        Point{x: x, y: y, z: z}
    }
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
        eq_f32(self.x, other.x) & eq_f32(self.y, other.y) & eq_f32(self.z, other.z)
    }
}
impl Eq for Point {}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        match self {
            _ if *self == *other => Ordering::Equal,
            _ if (self.x < other.x) |
                eq_f32(self.x,other.x) & (self.y < other.y) |
                eq_f32(self.x,other.x) & eq_f32(self.y,other.y) & (self.z < other.z) => Ordering::Less,
            _ => Ordering::Greater
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl BaseObject for Point {}


#[cfg(test)]
mod tests {
    use bo::vector::Vector;
    use bo::point::Point;

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
