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
    fn split(&mut self, ma : &Mesh, mb : &Mesh) -> (Vec<Triangle>, Vec<Triangle>, Vec<Triangle>, Vec<Triangle>) {
        let mut raf_simple_mxm : RafMxM = create();
        let ires = raf_simple_mxm.intersect(ma, mb);
        let raf_tr : RafTr = create();

        let mut a_map : HashMap<usize, TrD> = HashMap::new();
        let mut b_map : HashMap<usize, TrD> = HashMap::new();

        // нужно сгенерить структуры описатели треугольников!
        for ((index_a, index_b), res) in ires.map {
            let mut s : Segment = res.0.unwrap();

            if a_map.contains_key(&index_a) {
                let mut t_b : Triangle = mb.get_triangle(index_b);
                let mut normal = t_b.get_normal();
                a_map.get_mut(&index_a).unwrap().i_points.push(
                    (s.org.clone(), s.dest.clone(), normal)
                );
            } else {
                let mut t_a : Triangle = ma.get_triangle(index_a);
                let mut tr_points : Vec<Point> = vec![t_a.p1, t_a.p2, t_a.p3];
                let mut i_points : Vec<(Point, Point, Vector)> = Vec::new();

                let mut t_b : Triangle = mb.get_triangle(index_b);
                let mut normal = t_b.get_normal();
                i_points.push(
                    (s.org.clone(), s.dest.clone(), normal)
                );
                let desc = TrD {
                    tr_points: tr_points,
                    i_points: i_points
                };
                a_map.insert(index_a, desc);
            }

            if b_map.contains_key(&index_b) {
                let mut t_a : Triangle = ma.get_triangle(index_a);
                let mut normal = t_a.get_normal();
                b_map.get_mut(&index_b).unwrap().i_points.push(
                    (s.org, s.dest, normal)
                );
            } else {
                let mut t_b : Triangle = mb.get_triangle(index_b);
                let mut tr_points : Vec<Point> = vec![t_b.p1, t_b.p2, t_b.p3];
                let mut i_points : Vec<(Point, Point, Vector)> = Vec::new();
                let mut t_a : Triangle = ma.get_triangle(index_a);
                let mut normal = t_a.get_normal();
                i_points.push((s.org, s.dest, normal));
                let desc = TrD {
                    tr_points: tr_points,
                    i_points: i_points
                };
                b_map.insert(index_b, desc);
            }
            print!("{:?} {:?}\n", index_a, index_b);
        }
        let mut raf_tr : RafTr = create();

        //ia - intersection area
        //um - updated mesh
        //в моделе A заменяем все треугольники из зоны пересечения их триангуляцией
        println!("model A");
        let (uma, ia_a) : (Mesh, Vec<(usize, Marker)>) = create_um::<RafTr>(&ma, &a_map);
        //в моделе B заменяем все треугольники из зоны пересечения их триангуляцией
        println!("model B");
        let (umb, ia_b) : (Mesh, Vec<(usize, Marker)>) = create_um::<RafTr>(&mb, &b_map);

        let (inner_part_a, outer_part_a) = separate(&uma, ia_a);
        let (inner_part_b, outer_part_b) = separate(&umb, ia_b);

        /*
        println!("inner_part_a: {:?}", inner_part_a.len());
        println!("outer_part_a: {:?}", outer_part_a.len());
        println!("uma {:?}", uma.triangles.len());
        panic!();
        */
        return (inner_part_a, outer_part_a, inner_part_b, outer_part_b);
    }
    fn union(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp) {
        let (_, outer_part_a, _, outer_part_b) = self.split(ma, mb);
        let mut mesh : Mesh = Mesh::new();
        mesh.add_triangles(outer_part_a);
        mesh.add_triangles(outer_part_b);
        return (Some(mesh), InfoBoolOp::Intersecting);
    }

    fn intersect(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp) {
        let (inner_part_a, _, inner_part_b, _) = self.split(ma, mb);
        let mut mesh : Mesh = Mesh::new();
        mesh.add_triangles(inner_part_a);
        mesh.add_triangles(inner_part_b);
        return (Some(mesh), InfoBoolOp::Intersecting);
    }

    fn difference(&mut self, ma : &Mesh, mb : &Mesh) -> (Option<Mesh>, InfoBoolOp) {
        let (_, outer_part_a, inner_part_b, _) = self.split(ma, mb);
        let mut mesh : Mesh = Mesh::new();
        mesh.add_triangles(outer_part_a);
        mesh.add_triangles(inner_part_b);
        return (Some(mesh), InfoBoolOp::Intersecting);
    }
}



fn create_um<RafTr : AfTriangulate>(m : &Mesh, map : &HashMap<usize, TrD>) -> (Mesh, Vec<(usize, Marker)>) {
    let mut raf_tr : RafTr = create();
    let mut um : Mesh = Mesh::new();
    let mut ia : Vec<(usize, Marker)> = Vec::new();
    for index in 0 .. m.triangles.len() {
        if !map.contains_key(&index) {
            um.add_triangle(m.get_triangle(index));
        } else {
            let desc : &TrD = map.get(&index).unwrap();
            let ps : Vec<Point> = desc.extract_points();
            let ts = raf_tr.triangulate(ps);
            for mut t in ts {
                let pair = (um.triangles.len(), classify(&mut t, &desc));
                ia.push(pair);
                um.add_triangle(t);
            }
        }
    }
    return (um, ia);
}


fn classify(t : &mut Triangle, desc : &TrD) -> Marker {
    for &(ref p1, ref p2, ref normal) in &desc.i_points {
        // если какое-то звено кривой является стороной треугольника => классифицируем
        // 10 - волшебное число приносящее удачу
        let mut i1 : usize = 10;
        let mut i2 : usize = 10;
        for i in 1..4 {
            if p1.eq(t.get(i)) {
                i1 = i;
                continue;
            }
            if p2.eq(t.get(i)) {
                i2 = i;
                continue;
            }
        }

        if i1 != 10 && i2 != 10 {
            let j = 6 - i1 - i2;
            let x = (t.get(i1).x + t.get(i2).x)/2.;
            let y = (t.get(i1).y + t.get(i2).y)/2.;
            let z = (t.get(i1).z + t.get(i2).z)/2.;
            let middle = Point::new(x,y,z);
            let v : Vector = t.get(j) - &middle;
            let cp = v.dot_product(&normal);
            // TODO: figure out why sign is incorrect!
            //if cp > 0. {
            if cp < 0. {
                println!("Outer");
                return Marker::BoundOuter;
            } else {
                println!("Inner");
                return Marker::BoundInner;
            }
        }
    }
    println!("Bound");
    return Marker::Bound;
}

#[derive(Clone)]
#[derive(PartialEq, Eq)]
enum Marker {
    Unclassified,
    Inner,
    Outer,
    Bound,
    BoundInner,
    BoundOuter
}

fn separate(um : &Mesh, ia : Vec<(usize, Marker)>) -> (Vec<Triangle>, Vec<Triangle>) {
    let mut inner_part : Vec<Triangle> = Vec::new();
    let mut outer_part : Vec<Triangle> = Vec::new();

    let mut markers : Vec<Marker> = vec![Marker::Unclassified; um.triangles.len()];
    // заполняем маркеры уже известными
    for (i, marker) in ia.clone() {
        markers[i] = marker;
    }

    // Non-recursive dfs
    let mut nodes_to_visit : Vec<usize> = Vec::new();
    for (i, marker) in ia {
        if marker != Marker::Bound {
            let cur_marker : Marker;
            match marker.clone() {
                Marker::BoundInner => cur_marker = Marker::Inner,
                Marker::BoundOuter => cur_marker = Marker::Outer,
                _ => panic!("Smth goes wrong!")
            }
            nodes_to_visit.push(i);
            while !nodes_to_visit.is_empty() {
                let cur_index = nodes_to_visit.pop().unwrap();
                for ni in &um.triangles[cur_index].neighbors {
                    match markers[*ni].clone() {
                        Marker::Bound => {
                            if markers[cur_index] != Marker::BoundInner && markers[cur_index] != Marker::BoundOuter {
                                markers[*ni] = marker.clone();
                            }
                        },
                        //Marker::BoundInner | Marker::BoundOuter => 0,
                        //Marker::Inner | Marker::Outer => 0,
                        Marker::Unclassified => {
                            markers[*ni] = cur_marker.clone();
                            nodes_to_visit.push(*ni);
                        },
                        _ => ()
                    }
                }
            }
        }
    }
    let mut index = 0;
    for marker in markers {
        match marker {
            Marker::BoundInner | Marker::Inner => {
                inner_part.push(um.get_triangle(index));
            },

            Marker::BoundOuter | Marker::Outer => {
                outer_part.push(um.get_triangle(index));
            },

            _ => panic!("Incorrect triangle marker!")
        }
        index += 1;
    }

    return (inner_part, outer_part);
}

/*
fn dfs(
    um : &Mesh,
    inner : &Vec<Triangle>,
    outer : &Vec<Triangle>,
    ia : &BTreeSet<usize>,
    markers : & mut Vec<Markers>,
    i : usize
) {
    for ni in &um.triangles[i].neighbors {
        if !ia.contains(ni) {
            dfs(um, inner, outer, ia, markers, ni);
        }

    }
}
*/

//Triangle Descriptor
struct TrD {
    pub tr_points : Vec<Point>,
    pub i_points : Vec<(Point, Point, Vector)>
}

impl TrD {
    fn extract_points(&self) -> Vec<Point> {
        let mut v : Vec<Point> = Vec::new();
        for p in &self.tr_points {
            if !v.contains(&p) {
                v.push(p.clone());
            }

        }
        for &(ref p1,ref p2, _) in &self.i_points {
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
                let mut f = File::create("res_of_tests/simple_bool_op/union_test1.stl").unwrap();
                match m.write_stl(&mut f) {
                    Ok(_) => (),
                    Err(_) => panic!("Can not write into file!")
                };
            },
            Option::None => panic!("Wrong result!")
        }

    }

    #[test]
    fn first_intersect_test() {
        //cargo test first_union_test -- --nocapture
        let mut fa = File::open("input_for_tests/cube_in_origin.stl").unwrap();
        let mut fb = File::open("input_for_tests/scaled_shifted_cube.stl").unwrap();
        let ma : Mesh = read_stl(& mut fa).unwrap();
        let mb : Mesh = read_stl(& mut fb).unwrap();

        let mut raf_simple_bool_op : RafSimpleBoolOp = create();

        let (om, _) = raf_simple_bool_op.intersect(&ma, &mb);
        match om {
            Some(m) => {
                let mut f = File::create("res_of_tests/simple_bool_op/intersect_test1.stl").unwrap();
                match m.write_stl(&mut f) {
                    Ok(_) => (),
                    Err(_) => panic!("Can not write into file!")
                };
            },
            Option::None => panic!("Wrong result!")
        }

    }

    #[test]
    fn first_difference_test() {
        // TODO исправить нормали!!!
        let mut fa = File::open("input_for_tests/cube_in_origin.stl").unwrap();
        let mut fb = File::open("input_for_tests/scaled_shifted_cube.stl").unwrap();
        let ma : Mesh = read_stl(& mut fa).unwrap();
        let mb : Mesh = read_stl(& mut fb).unwrap();

        let mut raf_simple_bool_op : RafSimpleBoolOp = create();

        let (om, _) = raf_simple_bool_op.difference(&ma, &mb);
        match om {
            Some(m) => {
                let mut f = File::create("res_of_tests/simple_bool_op/difference_test1.stl").unwrap();
                match m.write_stl(&mut f) {
                    Ok(_) => (),
                    Err(_) => panic!("Can not write into file!")
                };
            },
            Option::None => panic!("Wrong result!")
        }

    }

}

