use bo::point::Point;
use bo::vector::Vector;
use std::fmt;
use bo::segment::Segment;

/*
TODO:
translate
rotate
intersect
*/

pub struct Line {
    pub org : Point,
    pub dest: Point
}

impl Line {
    pub fn convert_to_segment(self) -> Segment {
        Segment {org: self.org, dest: self.dest}
    }

    pub fn gen_segment(&self) -> Segment {
        Segment {org: self.org.clone(), dest: self.dest.clone()}
    }

    pub fn  get_dir_vector(&self) -> Vector {
        &self.dest - &self.org
    }

    pub fn check_accessory(&self, point : &Point) -> bool {
        let dir_vec = self.get_dir_vector();
        let check_vec = &self.org - point;
        let cp = dir_vec.cross_product(&check_vec);
        cp.is_zero()
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Line: {}, {})", self.org, self.dest)
    }
}