use std::ops::Add;
use std::ops::Sub;
use std::fmt;
use bo::vector::Vector;
use bo::{eq_f32, EPS};
use bo::base_object::BaseObject;
use std::cmp::Ordering;
use std::f32::consts::PI;
use std::f32;

#[derive(Debug)]
#[derive(Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(PartialEq, Eq)]
pub enum EClassify {
    Left,
    Right,
    Behind,
    Beyond,
    Org,
    Dest,
    Between
}

impl Point {
    pub fn convert_to_vector(self) -> Vector {
        Vector {x: self.x, y: self.y, z: self.z}
    }

    pub fn get_vector(&self) -> Vector {
        Vector {x: self.x, y: self.y, z: self.z}
    }

    pub fn new(x : f32, y : f32, z : f32) -> Point {
        Point{x: x, y: y, z: z}
    }

    pub fn swap_yz(& mut self) {
        let temp = self.y;
        self.y = self.z;
        self.z = temp;
    }

    pub fn swap_xy(& mut self) {
        let temp = self.x;
        self.x = self.y;
        self.y = temp;
    }

    pub fn swap_xz(& mut self) {
        let temp = self.x;
        self.x = self.z;
        self.z = temp;
    }

    pub fn classify(&self, p0 : &Point, p1 : &Point) -> EClassify {
        let a = p1 - p0;
        let b = self - p0;
        let sa = a.x*b.y - b.x*a.y;
        match 1 {
            _ if sa > EPS => return EClassify::Left,
            _ if sa < -EPS => return EClassify::Right,
            _ if (a.x * b.x < 0.0) | (a.y * b.y < 0.0) => return EClassify::Behind,
            _ if a.length() < b.length() => return EClassify::Beyond,
            _ if *p0 == *self => return EClassify::Org,
            _ if *p1 == *self => return EClassify::Dest,
            _ => return EClassify::Between
        }
    }

    pub fn rotate_around_axis(&mut self, point_on_axis : &Point, axis_dir : &Vector, angle : f32) {
        /*
        Point r = v - PointOnAxis;
        return PointOnAxis + cos(RadFromDeg(Angle)) * r
            + ((1 - cos(RadFromDeg(Angle))) * AxisDir.dotProduct3D(r)) * AxisDir
            + sin(RadFromDeg(Angle)) * AxisDir.crossProduct(r);
        */
        assert!(f32::abs(axis_dir.length() - 1.) <= EPS, "AxisDir must be unit vector");

        let res : Point;
        {
            let v: &Point = self;

            let r: Vector = v - point_on_axis;
            let part1: Point = point_on_axis + &(&r * rad_from_deg(angle).cos());
            let part2: f32 = axis_dir.dot_product(&r) * (1. - rad_from_deg(angle).cos());
            let part3: Vector = (&axis_dir.cross_product(&r)) * rad_from_deg(angle).sin();
            res = &(&part1 + &(axis_dir * part2)) + &part3;
        }
        *self = res

    }

}

fn rad_from_deg(x : f32) -> f32 {
    return (PI / 180.) * x;
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
