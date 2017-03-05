use cf::perform_bool_op::{AfBoolOp, InfoBoolOp};
use bo::*;
use cf::{Af, create};
use cf::triangulate::*;
use cf::intersect::*;
use qm::*;
use std::collections::{HashMap, BTreeSet};

#[derive(Default)]
pub struct RafSimpleBoolOp
<QM : QualityMetric = UselessQM,
    RafMxM : AfMxM = RafSimpleMxM,
    RafTr : AfTriangulate = RafIncrementalTriangulation>
{
    pub qm : QM,
    pub raf_mxm : RafMxM,
    pub raf_tr : RafTr
}

impl
<QM : QualityMetric,
    RafMxM : AfMxM,
    RafTr : AfTriangulate>
Af for RafSimpleBoolOp<QM, RafMxM, RafTr> {}


impl
<QM : QualityMetric,
    RafMxM : AfMxM,
    RafTr : AfTriangulate>
AfBoolOp for RafSimpleBoolOp<QM, RafMxM, RafTr> {
    fn union(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp) {
        let mut raf_simple_mxm : RafMxM = create();
        let ires = raf_simple_mxm.intersect(ma, mb);
        let raf_tr : RafTr = create();

        let mut mesh : Mesh = Mesh::new();
        let mut a_map : HashMap<usize, TrD> = HashMap::new();
        // let b_map : HashMap = HashMap::new();

        // только для выпуклых многогранников!
        for ((index_a, index_b), res) in ires.map {
            if a_map.contains_key(&index_a) {
                let mut s : Segment = res.0.unwrap();
                a_map.get_mut(&index_a).unwrap().i_points.push(
                    (index_b, (s.org, s.dest))
                );
            } else {
                let mut tr : Triangle = ma.get_triangle(index_a);
                let mut tr_points : Vec<Point> = vec![tr.p1, tr.p2, tr.p3];
                let mut i_points : Vec<(usize, (Point, Point))> = Vec::new();
                let s : Segment = res.0.unwrap();
                i_points.push(
                    (index_b, (s.org, s.dest))
                );
                let tr_d = TrD {
                    tr_points: tr_points,
                    i_points: i_points
                };
                a_map.insert(index_a, tr_d);
            }
            print!("{:?} {:?}\n", index_a, index_b);
        }
        let mut raf_tr : RafTr = create();

        for (index_a, tr_d) in a_map {
            println!("index_a: {:?}", index_a);
            println!("tr_points {:?}", tr_d.tr_points);
            println!("i_points {:?}", tr_d.i_points);
            let ps : Vec<Point> = tr_d.extract_points();
            let ts = raf_tr.triangulate(ps);
            for t in ts {
                mesh.add_triangle(t);
            }
            //break;
        }

        return (Some(mesh), InfoBoolOp::Intersecting);
    }
    fn intersect(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp) {
        return (None, InfoBoolOp::Intersecting);
    }
    fn difference(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp) {
        return (None, InfoBoolOp::Intersecting);
    }
}

//Triangle Descriptor
struct TrD {
    pub tr_points : Vec<Point>,
    pub i_points : Vec<(usize, (Point, Point))>
}

impl TrD {
    fn extract_points(&self) -> Vec<Point> {
        let mut v : Vec<Point> = Vec::new();
        for p in &self.tr_points {
            if !v.contains(&p) {
                v.push(p.clone());
            }

        }
        for &(_, (ref p1,ref p2)) in &self.i_points {
            if !v.contains(&p1) {
                v.push(p1.clone());
            }
            if !v.contains(&p2) {
                v.push(p2.clone());
            }
        }
        return v;
    }
}

#[cfg(test)]
mod tests {
    use cf::perform_bool_op::*;
    use cf::create;
    use bo::*;
    use std::option::Option;
    use std::fs::File;
    //use qm::*;

    //#[ignore]
    #[test]
    fn first_union_test() {
        //cargo test first_union_test -- --nocapture
        let mut fa = File::open("input_for_tests/cube_in_origin.stl").unwrap();
        let mut fb = File::open("input_for_tests/scaled_shifted_cube.stl").unwrap();
        let ma : Mesh = read_stl(& mut fa).unwrap();
        let mb : Mesh = read_stl(& mut fb).unwrap();

        let mut raf_simple_bool_op : RafSimpleBoolOp = create();

        let (om, _) = raf_simple_bool_op.union(&ma, &mb);
        match om {
            Some(m) => {
                let mut f = File::create("res_of_tests/simple_bool_op/test1.stl").unwrap();
                match m.write_stl(&mut f) {
                    Ok(_) => (),
                    Err(_) => panic!("Can not write into file!")
                };
            },
            Option::None => panic!("Wrong result!")
        }

    }
}












/*
A - first mesh
B - second mesh
A_in_ia
A_out_ia
B_in_ia
B_out_ia

//собираем полигоны
ia_polygons_A = new Map
ia_polygons_B = new Map
for (index1,index2) in IA.key:
    tr_a = A[index1]
    tr_b = B[index2]
    ires = IA[(index1,index2)]
    if index1 not in ia_polygons_A:
        ia_polygons_a[index1] = new Vec
        ia_polygons_a[index1].append(tr_a.p1)
        ia_polygons_a[index1].append(tr_a.p2)
        ia_polygons_a[index1].append(r_a.p3)
    if index2 not in ia_polygons_B:
        ia_polygons_b[index2] = new Vec
        ia_polygons_b[index2].append(tr_b.p1)
        ia_polygons_b[index2].append(tr_b.p2)
        ia_polygons_b[index2].append(tr_b.p3)

     ia_polygons_a[index1].append(ires.segment.org)
     ia_polygons_a[index1].append(ires.segment.dest)
     ia_polygons_b[index2].append(ires.segment.org)
     ia_polygons_b[index2].append(ires.segment.dest)


//классифицируем зону перечесчения
for (index1,index2) in IA.key:
    tr_a = A[index1]
    tr_b = B[index2]
    ires = IA[(index1,index2)]


//классифицируем остальные участки методом распространения






*/