use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::fmt;
use bo::eq_f32;
use bo::point::Point;

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

const ZERO : Vector = Vector {x: 0., y: 0., z:0.};

impl Vector {
    pub fn dot_product(&self, other: &Vector) -> f32 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }

    pub fn cross_product(&self, other: &Vector) -> Vector {
        //a2*b3  -   a3*b2,     a3*b1   -   a1*b3,     a1*b2   -   a2*b1
        Vector {x: self.y*other.z - self.z*other.y,
                y: self.z*other.x - self.x*other.z,
                z: self.x*other.y - self.y*other.x}
    }

    pub fn mixed_product(&self, a: &Vector, b: &Vector) -> f32 {
        self.dot_product(&(a.cross_product(b)))
    }

    pub fn is_it_zero(&self) -> bool {
        ZERO == *self
    }

    pub fn is_collinear_to(&self, other : &Vector) -> bool {
        self.cross_product(other).is_it_zero()
    }

    pub fn gen_point(&self) -> Point {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
            z: self.z.clone()
        }
    }
    pub fn new(x : f32, y : f32, z : f32) -> Vector {
        Vector {x:x, y:y, z:z}
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        eq_f32(self.x, other.x) & eq_f32(self.y, other.y) & eq_f32(self.z, other.z)
    }
}

impl Eq for Vector {}

impl<'a,'b> Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, other: &'b Vector) -> Vector {
        Vector { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z }
    }
}

impl<'a,'b> Sub<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn sub(self, other: &'b Vector) -> Vector {
        Vector { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z }
    }
}

impl<'a> Mul<f32> for &'a Vector {
    type Output = Vector;

    fn mul(self, other: f32) -> Vector {
        Vector { x: self.x*other, y: self.y*other, z: self.z*other }
    }
}


impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use bo::vector::Vector;
    use bo::eq_f32;
    //use super::super::point::Point;

    #[test]
    fn vector_plus_vector() {
        let v1 = Vector {x: 1.0, y: 1.0, z: 1.0};
        let v2 = Vector {x: 2.0, y: 1.0, z: 2.0};
        let new_v = &v1 + &v2;
        let expected_v = Vector {x: 3.0, y: 2.0, z: 3.0};
        assert!(new_v == expected_v);
    }

    #[test]
    fn vector_minus_vector() {
        let v1 = Vector {x: 1.0, y: 1.0, z: 1.0};
        let v2 = Vector {x: 2.0, y: 1.0, z: 2.0};
        let new_v = &v2 - &v1;
        let expected_v = Vector {x: 1.0, y: 0.0, z: 1.0};
        assert!(new_v == expected_v);
    }

    #[test]
    fn vector_dp_vector() {
        let v1 = Vector {x: 1.0, y: 1.0, z: 1.0};
        let v2 = Vector {x: 2.0, y: 1.0, z: 2.0};
        let dp = v2.dot_product(&v1);
        let expected_dp = 5.0;
        assert!(eq_f32(dp,expected_dp));
    }

    #[test]
    fn vector_cp_vector() {
        let v1 = Vector {x: 1.0, y: 1.0, z: 1.0};
        let v2 = Vector {x: 2.0, y: 1.0, z: 2.0};
        let d = v2.cross_product(&v1);
        let v1_dp_d = v1.dot_product(&d);
        let v2_dp_d = v2.dot_product(&d);
        assert!(eq_f32(v1_dp_d,0.0));
        assert!(eq_f32(v2_dp_d,0.0));
    }

    #[test]
    fn mp_of_three_vectors() {
        let a = Vector {x: 2.0, y: 0.0, z: 0.0};
        let b = Vector {x: 2.0, y: 1.0, z: 0.0};
        let c = Vector {x: 2.0, y: 1.0, z: 3.0};
        let mp_abc = a.mixed_product(&b, &c);
        let mp_cab = c.mixed_product(&a, &b);
        let mp_bca = b.mixed_product(&c, &a);
        assert!(eq_f32(mp_abc,mp_cab));
        assert!(eq_f32(mp_cab,mp_bca));
        assert!(eq_f32(mp_bca,mp_abc));
    }

}
