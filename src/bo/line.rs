use bo::point::Point;
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
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Line: {}, {})", self.org, self.dest)
    }
}