use bo::point::Point;
use bo::plane::Plane;
use bo::vector::Vector;

#[derive(Debug)]
pub struct Triangle {
    pub p1 : Point,
    pub p2 : Point,
    pub p3 : Point
}

impl Triangle {
    pub fn new(p1 : Point, p2 : Point, p3 : Point) -> Triangle {
        Triangle {
            p1 : p1,
            p2 : p2,
            p3 : p3
        }
    }

    pub fn gen_plane(&self) -> Plane {
        Plane {
            // check it!
            normal : (&self.p1 - &self.p2).cross_product(&(&self.p2 - &self.p3)),
            point : self.p1.clone()
        }
    }

    pub fn get_normal(&self) -> Vector {
        let v1 = &self.p1 - &self.p2;
        let v2 = &self.p2 - &self.p3;
        v1.cross_product(&v2)
    }
}


