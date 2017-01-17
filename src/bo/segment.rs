use bo::point::Point;
use std::fmt;




/*
TODO:
rotate
scale
translate
*/

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
pub struct Segment {
    pub org : Point,
    pub dest: Point
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Segment: {}, {})", self.org, self.dest)
    }
}

