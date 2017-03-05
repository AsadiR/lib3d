use bo::*;
use qm::*;
use std::collections::BTreeMap;
use cf::Af;
use cf::intersect::triangle_x_triangle::*;
use cf::enumerate::*;

type ResultTxT = (Option<Segment>,Option<Polygon>, InfoTxT);

// mesh intersection area
pub struct MeshIArea {
    pub map : BTreeMap<(usize,usize), ResultTxT>
}

/*
impl MeshIArea {
    fn take(&mut self, key : (usize, usize)) -> Option<ResultTxT> {
        self.map.remove(&key)
    }
}
*/

pub trait AfMxM : Af  {
    fn intersect(&mut self, a : &Mesh, b : &Mesh) -> MeshIArea;
}


#[derive(Default)]
pub struct RafSimpleMxM
<QM : QualityMetric = UselessQM,
    RafTxT : AfTxT = RafSimpleTxT,
    RafEnum : AfEnumTriangles = RafEnumSimple>
{
    pub qm : QM,
    pub raf_txt : RafTxT,
    pub raf_enum : RafEnum
}


impl
<QM : QualityMetric,
    RafTxT : AfTxT,
    RafEnum : AfEnumTriangles>
Af for RafSimpleMxM<QM, RafTxT, RafEnum> {}

impl
<QM : QualityMetric,
    RafTxT : AfTxT,
    RafEnum : AfEnumTriangles>
AfMxM for RafSimpleMxM<QM, RafTxT, RafEnum> {
    fn intersect(&mut self, a : &Mesh, b : &Mesh) -> MeshIArea {
        let triangles_enum = self.raf_enum.enumerate(a, b);
        let mut map : BTreeMap<(usize,usize), ResultTxT> = BTreeMap::new();
        for (index_a, index_b) in triangles_enum {
            let tr_a = a.get_triangle(index_a);
            let tr_b = b.get_triangle(index_b);
            let res_txt = self.raf_txt.intersect(&tr_a, &tr_b);
            if res_txt.2 == InfoTxT::Intersecting {
                //TODO: (res_txt.2 == InfoTxT::CoplanarIntersecting)
                map.insert((index_a, index_b), res_txt);
            }
        }
        MeshIArea {
            map : map,
        }
    }
}