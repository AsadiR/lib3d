use super::*;
use qm::*;
use cf::Af;

#[derive(Default)]
pub struct RafEnumSimple<QM : QualityMetric = UselessQM> {
    pub qm : QM
}

impl<QM : QualityMetric>  Af for RafEnumSimple<QM> {}

impl<QM : QualityMetric> AfEnumTriangles for RafEnumSimple<QM> {
    fn enumerate(&mut self, a : &Mesh, b : &Mesh) -> TupleIter {
        let mut v : Vec<(usize, usize)> = Vec::new();
        for index_a in 0..a.triangles.len() {
            for index_b in 0..b.triangles.len() {
                v.push((index_a,index_b));
            }
        }
        TupleIter::new(v)
    }
}


