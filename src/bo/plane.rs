use bo::vector::Vector;
use bo::point::Point;

/*
TODO:
intersect
translate
rotate
*/


// n*(p-p0) = 0
pub struct Plane {
    pub normal: Vector,
    pub point: Point,
}

impl Plane {

    // n*x + d = 0
    pub fn get_d(&self) -> f32{
        -self.normal.dot_product(&self.point.get_vector())
    }
}