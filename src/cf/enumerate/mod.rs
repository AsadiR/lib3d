mod raf_enum_simple;

use cf::Af;
use bo::*;
pub use self::raf_enum_simple::RafEnumSimple;
//pub use self::simple_iter;

pub struct TupleIter {
    v : Vec<(usize, usize)>,
    index : usize
}

impl Iterator for TupleIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        if self.index >= self.v.len() {
            return None;
        }
        let value = self.v[self.index];
        self.index += 1;
        Some(value)
    }
}

impl TupleIter {
    fn new(v : Vec<(usize, usize)>) -> TupleIter {
        TupleIter {v: v, index: 0}
    }

}

pub trait AfEnumTriangles : Af  {
    fn enumerate(&mut self, a : &Mesh, b : &Mesh) -> TupleIter;

}
